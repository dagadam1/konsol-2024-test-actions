use crate::actions;
use crate::auth;
use crate::auth::PermissionLevel;
use crate::fs_helpers;
use crate::models::{User, Settings};

use super::auth::check_user_permission;

use actix_web::error;
use actix_web::get;
use actix_web::post;
use actix_web::web;
use google_oauth::AsyncClient;

use actix_session::Session;
use serde::Deserialize;
use serde::Serialize;

use super::auth::AuthenticatedUser;

use actix_web::HttpResponse;

use actix_web::error::ErrorInternalServerError;

use uuid::Uuid;

use actix_web::Responder;

use super::SlideUploadForm;

use actix_multipart::form::MultipartForm;

use super::DbPool;

// For documentation, see endpoints.md

// A route is protected (needs auth) if it has a parameter of type AuthenticatedUser

// --- Slides ---

#[post("/api/screen/slides/save")]
pub(crate) async fn save_slide(
    pool: web::Data<DbPool>,
    form: MultipartForm<SlideUploadForm>,
) -> actix_web::Result<impl Responder> {
    let id = Uuid::new_v4();

    // Parse the form into a Slide and a TempFile (the image)
    let (slide, image_file) = form.into_inner().parse_form(id).map_err(ErrorInternalServerError)?;

    // Save file to disk
    let image_path = fs_helpers::save_image_file(image_file, &String::from(id), &slide.filetype).await?;

    // Add Slide to database
    let db_result = web::block(move || {
        let mut conn = pool.get()?;

        actions::insert_slide(&mut conn, slide)
    })
    .await?;

    // Return different responses depending on if the database succeeded or not
    match db_result {
        Ok(added_slide) => Ok(HttpResponse::Created().json(added_slide)),
        Err(e) => {
            // If the database failed, remove the file from disk
            fs_helpers::remove_file(image_path).await?;
            // Map the error to an internal server error
            Err(ErrorInternalServerError(e))
        }
    }
}

#[get("/api/screen/slides")]
pub(crate) async fn get_slides(
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {

    let all_slides = web::block(move || {
        let mut conn = pool.get()?;
        actions::get_all_slides(&mut conn)
    }).await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(all_slides))
}

// --- Authentication ---

#[derive(Serialize, Deserialize, Debug)]
struct AuthRequest {
    id_token: String,
}

#[post("/api/auth/verify")]
pub(crate) async fn verify_token(req: web::Json<AuthRequest>, session: Session, pool: web::Data<DbPool>) -> HttpResponse {

    let client_id = std::env::var("GOOGLE_ID_TOKEN").expect("GOOGLE_ID_TOKEN should be set");
    
    let client = AsyncClient::new(&client_id);
    let payload = match client.validate_id_token(&req.id_token).await {
        Ok(g) => g,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    let Some(email) = payload.email else { 
        log::error!("No email in Google payload");
        return HttpResponse::InternalServerError().finish();
    };

    let permission = match check_user_permission(email.clone(), pool).await {
        Ok(Some(permission)) => {permission},
        Ok(None) => {return HttpResponse::Unauthorized().finish()},
        Err(e) => {
            log::error!("check_user_permission error: {e}");
            return HttpResponse::InternalServerError().finish()
        },
    };

    let user = AuthenticatedUser { email, permission };

    match session.insert("auth", &user) {
        Ok(()) => {
            log::info!("User {} authenticated", user.email);
            session.renew();
            HttpResponse::Ok().json(user)
        },
        Err(e) => {
            log::error!("{}", e);
            HttpResponse::InternalServerError().finish()
        },
    }
}

#[get("/api/auth/status")]
pub(crate) async fn login_status(user: AuthenticatedUser) -> HttpResponse {
    // This only runs if the user is authenticated, see AuthenticatedUser
    HttpResponse::Ok().json(user)
}

#[post("/api/auth/logout")]
pub(crate) async fn logout(_: AuthenticatedUser, session: Session) -> HttpResponse {
    session.clear();
    HttpResponse::Ok().finish()
}

#[derive(Debug, Serialize, Deserialize)]
struct AddUserRequest {
    email: String,
    permission: PermissionLevel,
}

#[post("/api/auth/add_user")]
pub(crate) async fn add_user(user_req: web::Json<AddUserRequest>, caller: AuthenticatedUser, pool: web::Data<DbPool>) 
-> actix_web::Result<HttpResponse> {
    // Check if caller has admin permissions
    if let AuthenticatedUser { permission: PermissionLevel::Admin, .. } = caller {
        // Use web::block to avoid blocking async
        let user = web::block(move || {
            let mut conn = pool.get()?;

            // Parse AddUserRequest to User
            let user = User {
                id: Uuid::new_v4().into(),
                email: user_req.email.clone(),
                admin: match user_req.permission {
                    PermissionLevel::User => false,
                    PermissionLevel::Admin => true,
                },
            };

            actions::insert_user(&mut conn, user)
        }).await?.map_err(error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok().json(user))
    } else {
        Ok(HttpResponse::Forbidden().finish())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RemoveUserRequest {
    id: String,
}

#[post("/api/auth/remove_user")]
pub(crate) async fn remove_user(user_req: web::Json<RemoveUserRequest>, caller: AuthenticatedUser, pool: web::Data<DbPool>)
-> actix_web::Result<HttpResponse> {
    // Check if caller has admin permissions
    if let AuthenticatedUser { permission: PermissionLevel::Admin, .. } = caller {
        // Use web::block to avoid blocking async
        web::block(move || {
            let mut conn = pool.get()?;

            actions::remove_user(&mut conn, &user_req.id)
        }).await?.map_err(error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::Forbidden().finish())
    }
}

#[get("/api/auth/list_users")]
pub(crate) async fn list_users(caller: AuthenticatedUser, pool: web::Data<DbPool>) -> actix_web::Result<HttpResponse> {
    // Check if caller has admin permissions
    if let AuthenticatedUser { permission: PermissionLevel::Admin, .. } = caller {
        // Use web::block to avoid blocking async
        let users = web::block(move || {
            let mut conn = pool.get()?;

            actions::get_all_users(&mut conn)
        }).await?.map_err(error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok().json(users))
    } else {
        Ok(HttpResponse::Forbidden().finish())
    }
}


// --- Settings ---
#[derive(Debug, Serialize, Deserialize)]
struct SettingsResponse {
    pub layout_type: String,
    pub color_mode: String,
}
impl From<Settings> for SettingsResponse {
    fn from(settings: Settings) -> Self {
        SettingsResponse {
            layout_type: settings.layout_type,
            color_mode: settings.color_mode,
        }
    }
}

#[get("/api/screen/settings")]
pub(crate) async fn get_settings(pool: web::Data<DbPool>) -> actix_web::Result<HttpResponse> {
    // Use web::block to avoid blocking async
    let settings = web::block(move || {
        let mut conn = pool.get()?;

        actions::get_settings(&mut conn)
    }).await?.map_err(error::ErrorInternalServerError)?;

    let settings_response: SettingsResponse = settings.into();

    Ok(HttpResponse::Ok().json(web::Json(settings_response)))
}
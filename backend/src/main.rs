// This code is based on this example from Actix Web: 
// https://github.com/actix/examples/tree/master/databases/diesel
// I only kept the routes and tests here for example purposes and they are not 
// supposed to be part of the final project.

#[macro_use]
extern crate diesel;

use std::{borrow::Borrow, fs::File, path::{self, Path, PathBuf}};

use actix_web::{error::{self, ErrorInternalServerError}, get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use actix_multipart::form::{tempfile::TempFile, MultipartForm, text::Text};
use actix_cors::Cors;
use diesel::{prelude::*, r2d2};
use uuid::Uuid;

mod actions;
mod models;
mod schema;

/// Short-hand for the database pool type to use throughout the app.
type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

// No trailing slash
const SLIDE_IMAGE_DIR: &str = "/tmp/konsol_slides";

#[derive(Debug, MultipartForm)]
struct SlideUploadForm {
    caption: Text<String>,
    start: Text<String>,
    end: Text<String>,
    visible: Text<bool>,
    fullscreen: Text<bool>,
    #[multipart(rename = "imageFile")]
    image_file: TempFile,
    #[multipart(rename = "tags[]")]
    tags: Text<String>,
}

impl SlideUploadForm {
    fn slide_and_image_from_form(self, id: Uuid) -> Result<(models::Slide, TempFile), chrono::ParseError> {
        Ok((
            models::Slide {
                id: id.into(),
                caption: self.caption.into_inner(),
                start_date: self.start.into_inner().parse()?,
                end_date: self.end.into_inner().parse()?,
                active: self.visible.into_inner(),
            },
        
            self.image_file
        ))
    }
}

async fn save_image_file(
    temp_file: TempFile,
    filename: &str,
) -> actix_web::Result<()> {
    let file_path: PathBuf = [SLIDE_IMAGE_DIR, filename].iter().collect();

    web::block(move || temp_file.file.persist(file_path))
        .await?
        .map_err(|e| {
            eprintln!("file error: {:?}", e);
            error::ErrorInternalServerError(e)
    })?;

    Ok(())
}

async fn remove_image_file(
    filename: &str
) -> actix_web::Result<()> {
    let file_path: PathBuf = [SLIDE_IMAGE_DIR, filename].iter().collect();

    web::block(move || {
        std::fs::remove_file(file_path)
    })
    .await
    .map_err(|e| {
        eprintln!("file error: {:?}", e);
        error::ErrorInternalServerError(e)
    })?;

    Ok(())
}

#[post("/api/screen/slides/save")]
async fn save_slide(
    pool: web::Data<DbPool>,
    form: MultipartForm<SlideUploadForm>,
) -> actix_web::Result<impl Responder> {
    let id = Uuid::new_v4();

    let (slide, image_file) = form.into_inner().slide_and_image_from_form(id).map_err(ErrorInternalServerError)?;


    // Save file to disk
    save_image_file(image_file, &String::from(id)).await?;

    // Add slide to database
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
            remove_image_file(&String::from(id)).await?;
            // Map the error to an internal server error
            Err(ErrorInternalServerError(e))
        }
    }
}

/// Finds user by UID.
///
/// Extracts:
/// - the database pool handle from application data
/// - a user UID from the request path
#[get("/user/{user_id}")]
async fn get_user(
    pool: web::Data<DbPool>,
    user_uid: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let user_uid = user_uid.into_inner();

    // use web::block to offload blocking Diesel queries without blocking server thread
    let user = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::find_user_by_uid(&mut conn, user_uid)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(match user {
        // user was found; return 200 response with JSON formatted user object
        Some(user) => HttpResponse::Ok().json(user),

        // user was not found; return 404 response with error message
        None => HttpResponse::NotFound().body(format!("No user found with UID: {user_uid}")),
    })
}

/// Creates new user.
///
/// Extracts:
/// - the database pool handle from application data
/// - a JSON form containing new user info from the request body
#[post("/user")]
async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> actix_web::Result<impl Responder> {
    // use web::block to offload blocking Diesel queries without blocking server thread
    let user = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::insert_new_user(&mut conn, &form.name)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    // user was added successfully; return 201 response with new user info
    Ok(HttpResponse::Created().json(user))
}

#[get("/api/slides")]
async fn get_slides(
    pool: web::Data<DbPool>
) -> actix_web::Result<impl Responder> {
    
    let all_slides = web::block(move || {
        let mut conn = pool.get()?;
        actions::get_all_slides(&mut conn)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;
    
    Ok(HttpResponse::Ok().json(all_slides))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // initialize DB pool outside of `HttpServer::new` so that it is shared across all workers
    let pool = initialize_db_pool();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // TODO: restrict cors
            .allow_any_method()
            .allow_any_header();

        App::new()
            // add DB pool handle to app data; enables use of `web::Data<DbPool>` extractor
            .app_data(web::Data::new(pool.clone()))
            // add request logger middleware
            .wrap(middleware::Logger::default())
            .wrap(cors)
            // add route handlers
            .service(get_user)
            .service(add_user)
            .service(get_slides)
            .service(save_slide)
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

/// Initialize database connection pool based on `DATABASE_URL` environment variable.
///
/// See more: <https://docs.rs/diesel/latest/diesel/r2d2/index.html>.
fn initialize_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};
    use std::fs;

    use super::*;

    #[actix_web::test]
    async fn test_slide_endpoints() {
        dotenvy::dotenv().ok();
        env_logger::try_init_from_env(env_logger::Env::new().default_filter_or("info")).ok();

        let pool = initialize_db_pool();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .wrap(middleware::Logger::default())
                .service(save_slide)
                .service(get_slides),
        )
        .await;
//api/screen/slides/save

        // Make a new slide

        let test_request = fs::read("src/test_request").expect("Unable to read file");
        
        let req = test::TestRequest::post().uri("/api/screen/slides/save").set_payload(test_request).to_request();
        

        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::OK);

        // let body = test::read_body(res).await;
        // assert!(
        //     body.starts_with(b"UUID parsing failed"),
        //     "unexpected body: {body:?}",
        // );

        // try to find a non-existent user
        // let req = test::TestRequest::get()
        //     .uri(&format!("/user/{}", Uuid::nil()))
        //     .to_request();
        // let res = test::call_service(&app, req).await;
        // assert_eq!(res.status(), StatusCode::NOT_FOUND);
        // let body = test::read_body(res).await;
        // assert!(
        //     body.starts_with(b"No user found"),
        //     "unexpected body: {body:?}",
        // );

        // // create new user
        // let req = test::TestRequest::post()
        //     .uri("/user")
        //     .set_json(models::NewUser::new("Test user"))
        //     .to_request();
        // let res: models::User = test::call_and_read_body_json(&app, req).await;
        // assert_eq!(res.name, "Test user");

        // // get a user
        // let req = test::TestRequest::get()
        //     .uri(&format!("/user/{}", res.id))
        //     .to_request();
        // let res: models::User = test::call_and_read_body_json(&app, req).await;
        // assert_eq!(res.name, "Test user");

        // // delete new user from table
        // use crate::schema::users::dsl::*;
        // diesel::delete(users.filter(id.eq(res.id)))
        //     .execute(&mut pool.get().expect("couldn't get db connection from pool"))
        //     .expect("couldn't delete test user from table");
    }
}

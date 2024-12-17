// This code is based on this example from Actix Web: 
// https://github.com/actix/examples/tree/master/databases/diesel
// I only kept the routes and tests here for example purposes and they are not 
// supposed to be part of the final project.

#[macro_use]
extern crate diesel;

use std::{any::Any, borrow::Borrow, fs::{self, File}, io, path::{self, Path, PathBuf}};

use actix_web::{error::{self, ErrorInternalServerError}, get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use actix_multipart::form::{tempfile::TempFile, MultipartForm, text::Text};
use actix_cors::Cors;
use actix_files;
use chrono::{NaiveDate, NaiveTime};
use diesel::{prelude::*, r2d2};
use uuid::Uuid;

mod actions;
mod models;
mod schema;

/// Short-hand for the database pool type to use throughout the app.
type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

// The directory where slide images are saved. Will be created if it does not exist. This whole directory gets served as static files.
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
                // Since the form currently only has date input (but the db stores both date and time), we set the time to 00:00:00
                // To do this, we first parse the date into a NaiveDate, and then add the time with NaiveDate::and_time
                start_date: self.start.into_inner().parse::<NaiveDate>()?.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
                end_date: self.end.into_inner().parse::<NaiveDate>()?.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
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
    let mime = match temp_file.content_type {
        Some(mime) => {mime},
        None => {return Err(error::ErrorInternalServerError("No content type found for file"))},
    };
    
    // The file path is the SLIDE_IMAGE_DIR + filename. This is colleted into a PathBuf
    let mut file_path: PathBuf = [SLIDE_IMAGE_DIR, filename].iter().collect();
    // The MIME subtype determines the file extension
    file_path.set_extension(mime.subtype().as_str());

    log::info!("Saving image as: {:?}", file_path);
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
    .await?
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

#[get("/api/screen/slides")]
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


// #[get("/api/slides/{slide_id}/img")]
// async fn get_slide_img(
//     pool: web::Data<DbPool>,
//     slide_id: web::Path<Uuid>,
// ) -> actix_web::Result<impl Responder> {
//     let slide_id = slide_id.into_inner();

//     // Image path will be the string SLIDE_IMAGE_DIR + slide_id. This is turned into a PathBuf
//     let img_path = [SLIDE_IMAGE_DIR, &String::from(slide_id)].iter().collect::<PathBuf>();
    
//     // We don't need to check if the slide exists in the database, we only need to try to find the image file (this is fine right?)
//     // Reading a file is blocking, so we offload it to a thread
//     web::block(move || {
//         match fs::read(img_path) {
//             // TODO. File type should not be hardcoded to jpeg!
//             Ok(img) => HttpResponse::Ok().content_type("image/jpeg").body(img),
//             Err(e) => {
//                 if let io::ErrorKind::NotFound = e.kind() {
//                     HttpResponse::NotFound().body(format!("No image found for slide with ID: {slide_id}"))
//                 } else {
//                     HttpResponse::InternalServerError().body("Error")
//                 }
//             },
//         }
//     });

//     match slide {
//         Some(slide) => {
//             let file_path: PathBuf = [SLIDE_IMAGE_DIR, &slide.id].iter().collect();
//             let file = File::open(file_path).map_err(|e| {
//                 eprintln!("file error: {:?}", e);
//                 error::ErrorInternalServerError(e)
//             })?;

//             Ok(HttpResponse::Ok().content_type("image/jpeg").streaming(file))
//         },
//         None => Ok(HttpResponse::NotFound().body(format!("No slide found with ID: {slide_id}"))),
//     }
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // initialize DB pool outside of `HttpServer::new` so that it is shared across all workers
    let pool = initialize_db_pool();

    // If SLIDE_IMAGE_DIR does not exist, create it
    if !fs::exists(SLIDE_IMAGE_DIR).expect("Unable to check if slide image directory exists") {
        log::info!("Creating slide image directory at {SLIDE_IMAGE_DIR}");
        fs::create_dir_all(SLIDE_IMAGE_DIR).expect(&format!("Unable to create slide image directory at {}", SLIDE_IMAGE_DIR));
    }

    log::info!("saving images at {SLIDE_IMAGE_DIR}");

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
            .service(actix_files::Files::new("/slides", SLIDE_IMAGE_DIR))
            
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

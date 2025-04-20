#[macro_use]
extern crate diesel;

use std::{ fs, path::PathBuf};

use actix_web::{error::{self, ErrorInternalServerError}, get, delete, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use actix_multipart::form::{tempfile::TempFile, MultipartForm, text::Text};
use actix_cors::Cors;
use chrono::{NaiveDate, NaiveTime};
use diesel::{prelude::*, r2d2};
use models::Slide;
use uuid::Uuid;

mod actions;
mod models;
mod schema;

/// Short-hand for the database pool type to use throughout the app.
type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

// The directory where slide images are saved. Will be created if it does not exist. This whole directory gets served as static files.
// No trailing slash
const SLIDE_IMAGE_DIR: &str = "./slide_images";

#[derive(Debug, MultipartForm)]
struct SlideUploadForm {
    caption: Text<String>,
    start: Text<String>,
    end: Text<String>,
    visible: Text<bool>,
    #[multipart(rename = "imageFile")]
    image_file: TempFile,
}

impl SlideUploadForm {
    fn parse_form(self, id: Uuid) -> Result<(models::Slide, TempFile), actix_web::Error> {
        
        // Get the MIME type of the file. This determines the file extension
        let mime = match self.image_file.content_type {
            Some(ref mime) => mime,
            None => {return Err(error::ErrorInternalServerError("No content type found for uploaded file"))},
        };

        Ok((
            models::Slide {
                id: id.into(),
                caption: self.caption.into_inner(),
                // Since the form currently only has date input (but the db stores both date and time), we set the time to 00:00:00
                // To do this, we first parse the date into a NaiveDate, and then add the time with NaiveDate::and_time
                start_date: self.start.into_inner().parse::<NaiveDate>()
                    .map_err(error::ErrorInternalServerError)?
                    .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
                end_date: self.end.into_inner().parse::<NaiveDate>()
                    .map_err(error::ErrorInternalServerError)?
                    .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
                active: self.visible.into_inner(),
                filetype: mime.subtype().to_string(),
            },
        
            self.image_file
        ))
    }
}

fn get_image_path(slide: &Slide) -> PathBuf {
    let mut path: PathBuf = [SLIDE_IMAGE_DIR, &slide.id].iter().collect();
    path.set_extension(&slide.filetype);
    path
}

async fn save_image_file(
    temp_file: TempFile,
    path: PathBuf,
) -> actix_web::Result<()> {
    log::info!("Saving image as: {:?}", &path);
    
    // Saving the file is potentially blocking, so we use web::block to offload it to a threadpool
    // There might be problems if the file is being read somwhere else while it is only partially written.
    // I don't think we have to worry about it, but it can probably be fixed by writing a temporary file and then moving it once done
    web::block(move || temp_file.file.persist(path))
    .await?
    .map_err(|e| {
        eprintln!("file error: {:?}", e);
        error::ErrorInternalServerError(e)
    })?;

    Ok(())
}

async fn remove_file(
    path: PathBuf,
) -> actix_web::Result<()> {
    log::info!("Removing file at: {:?}", path);

    // Removing the file is potentially blocking, so we use web::block to offload it to a threadpool
    web::block(move || {
        std::fs::remove_file(path)
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

    // Parse the form into a Slide and a TempFile (the image)
    let (slide, image_file) = form.into_inner().parse_form(id).map_err(ErrorInternalServerError)?;

    let path = get_image_path(&slide);

    // Save file to disk
    save_image_file(image_file, path.clone()).await?;

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
            remove_file(path).await?;
            // Map the error to an internal server error
            Err(ErrorInternalServerError(e))
        }
    }
}

#[delete("/api/screen/slides/{id}")]
async fn remove_slide(
    id: web::Path<String>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    println!("{:?}", id);
    let uuid = Uuid::parse_str(&id).map_err(|e| {
        eprintln!("invalid uuid: {:?}", e);
        error::ErrorInternalServerError(e)
    })?;

    let db_result = web::block(move || {
        let mut conn = pool.get()?;

        actions::pop_slide(&mut conn, &uuid)
    }).await?;
    
    let slide = db_result.map_err(ErrorInternalServerError)?;

    let path = get_image_path(&slide);

    remove_file(path).await?;

    Ok(HttpResponse::Ok().finish())
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
            .service(save_slide)
            .service(get_slides)
            .service(get_slides)
            .service(remove_slide)
            .service(actix_files::Files::new("/api/screen/slides/images",SLIDE_IMAGE_DIR))

            // add route handlers
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

    use super::*;

    #[actix_web::test]
    async fn test_get_slides_returns_ok() {
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

        let req1 = test::TestRequest::get().uri("/api/screen/slides").to_request();
        let res1 = test::call_service(&app, req1).await;
        assert_eq!(res1.status(), StatusCode::OK);
    }
}

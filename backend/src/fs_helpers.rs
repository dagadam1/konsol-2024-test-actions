use actix_web::{error::ErrorInternalServerError, web};

use super::SLIDE_IMAGE_DIR;

use std::path::PathBuf;

use actix_multipart::form::tempfile::TempFile;

pub(crate) async fn save_image_file(
    temp_file: TempFile,
    filename: &str,
    file_type: &str,
) -> actix_web::Result<PathBuf> {
    // The filetype can be determined from the data in the TempFile itself, but since we've already had to determine it earlier,
    // we can pass it in as an argument instead

    // The file path is the SLIDE_IMAGE_DIR + filename. This is colleted into a PathBuf
    let mut file_path: PathBuf = [SLIDE_IMAGE_DIR, filename].iter().collect();
    // The file extension is the filetype
    file_path.set_extension(file_type);

    let saved_path = file_path.clone(); // We need to clone the path because we want to return it later

    log::info!("Saving image as: {:?}", file_path);

    // Saving the file is potentially blocking, so we use web::block to offload it to a threadpool
    // There might be problems if the file is being read somwhere else while it is only partially written.
    // I don't think we have to worry about it, but it can probably be fixed by writing a temporary file and then moving it once done
    web::block(move || temp_file.file.persist(file_path))
    .await?
    .map_err(|e| {
        eprintln!("file error: {:?}", e);
        ErrorInternalServerError(e)
    })?;

    // Return the path to the saved file
    Ok(saved_path)
}

pub(crate) async fn remove_file(
    file_path: PathBuf,
) -> actix_web::Result<()> {
    log::info!("Removing file at: {:?}", file_path);

    // Removing the file is potentially blocking, so we use web::block to offload it to a threadpool
    web::block(move || {
        std::fs::remove_file(file_path)
    })
    .await?
    .map_err(|e| {
        eprintln!("file error: {:?}", e);
        ErrorInternalServerError(e)
    })?;

    Ok(())
}

use diesel::prelude::*;
use uuid::Uuid;

use crate::models::{self, Slide};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_all_slides(
    conn: &mut SqliteConnection,
) -> Result<Vec<models::Slide>, DbError> {
    use crate::schema::slides::dsl::*;

    let all_slides = slides.load::<models::Slide>(conn)?;

    Ok(all_slides)
}

pub fn insert_slide(
    conn: &mut SqliteConnection,
    slide: models::Slide,
) -> Result<models::Slide, DbError> {
    use crate::schema::slides::dsl::*;

    diesel::insert_into(slides)
        .values(&slide)
        .execute(conn)?;

    Ok(slide)
}

pub fn pop_slide(
    conn: &mut SqliteConnection,
    uuid: &Uuid,
) -> Result<Slide,DbError> {
    use crate::schema::slides::dsl::*;

    // Get slide
    let slide = slides
        .find(uuid.to_string())
        .first(conn)?;
    
    // Delete slide
    diesel::delete(slides.filter(id.eq(uuid.to_string())))
        .execute(conn)?;

    Ok(slide)
}
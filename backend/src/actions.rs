use std::io;

use diesel::prelude::*;
use uuid::Uuid;

use crate::models;

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


/// Run query using Diesel to find user by uid and return it.
pub fn find_user_by_uid(
    conn: &mut SqliteConnection,
    uid: Uuid,
) -> Result<Option<models::User>, DbError> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(uid.to_string()))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_user(
    conn: &mut SqliteConnection,
    nm: &str, // prevent collision with `name` column imported inside the function
) -> Result<models::User, DbError> {
    // It is common when using Diesel with Actix Web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::users::dsl::*;

    let new_user = models::User {
        id: Uuid::new_v4().to_string(),
        name: nm.to_owned(),
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}
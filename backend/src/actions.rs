use diesel::prelude::*;

use crate::models::{self, Slide};

use crate::models::User;
use crate::auth::PermissionLevel;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_all_slides(
    conn: &mut SqliteConnection,
) -> Result<Vec<Slide>, DbError> {
    use crate::schema::slides::dsl::*;

    let all_slides = slides.load::<Slide>(conn)?;

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

pub fn insert_user(conn: &mut SqliteConnection, user: User) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;

    // Silently ignores users with emails already in the db
    diesel::insert_into(users)
        .values(&user)
        .on_conflict(email)
        .do_nothing()
        .execute(conn)?;

    Ok(user)
}

pub fn check_email_permission(conn: &mut SqliteConnection, email_str: &str) -> Result<Option<PermissionLevel>, DbError> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(email.eq(email_str.to_owned()))
        .first::<models::User>(conn)
        .optional()?;

    Ok(
        match user {    
            Some(User { admin: true, .. }) => Some(PermissionLevel::Admin),
            Some(User { admin: false, .. }) => Some(PermissionLevel::User),
            None => None,
        }
    )
}
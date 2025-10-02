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

/// Insert a user into the database and return it
pub fn insert_user(conn: &mut SqliteConnection, user: User) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;

    // Check if the user already exists
    let existing_user = users
        .filter(email.eq(user.email.clone()))
        .first::<User>(conn)
        .optional()?;

    // If the user already exists, return it instead
    if let Some(user) = existing_user {
        return Ok(user);
    }

    diesel::insert_into(users)
        .values(&user)
        .on_conflict(email)
        .do_nothing()
        .execute(conn)?;

    Ok(user)
}

pub fn remove_user(conn: &mut SqliteConnection, user_id: &str) -> Result<(), DbError> {
    use crate::schema::users::dsl::*;

    diesel::delete(users.filter(id.eq(user_id)))
        .execute(conn)?;

    Ok(())
}

pub fn get_all_users(
    conn: &mut SqliteConnection,
) -> Result<Vec<User>, DbError> {
    use crate::schema::users::dsl::*;

    let all_users = users.load::<User>(conn)?;

    Ok(all_users)
}

/// Check a user against the database
/// Returns None if the user does not exist, otherwise a PermissionLevel
pub fn check_user(conn: &mut SqliteConnection, email_str: &str) -> Result<Option<PermissionLevel>, DbError> {
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

pub fn get_settings(conn: &mut SqliteConnection) -> Result<models::Settings, DbError> {
    use crate::schema::settings::dsl::*;

    log::info!("Fetching settings from database");
    let setting = settings.first::<models::Settings>(conn);
    log::info!("Fetched settings from database: {:?}", setting);
    let setting = setting?;

    log::info!("Fetched settings from database: {:?}", setting);
    Ok(setting)
}
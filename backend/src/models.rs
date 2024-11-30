use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::schema::{users, slides};

/// User details.
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub name: String,
}

/// New user details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, QueryableByName)]
#[diesel(table_name = slides)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Slide {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image_path: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub active: bool,
}

#[derive(Insertable)]
#[diesel(table_name = slides)]
pub struct NewSlide {
    pub title: String,
    pub description: String,
    pub image_path: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
}

impl NewUser {
    /// Constructs new user details from name.
    #[cfg(test)] // only needed in tests
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

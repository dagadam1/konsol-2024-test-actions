use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::schema::{ slides, users, settings };

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, QueryableByName, Insertable)]
#[diesel(table_name = slides)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[cfg_attr(test, derive(PartialEq))] // Only derive `PartialEq` for tests
pub struct Slide {
    pub id: String,
    pub caption: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub active: bool,
    pub filetype: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, QueryableByName, Insertable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: String,
    pub email: String,
    pub admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, QueryableByName, Insertable, Selectable)]
#[diesel(table_name = settings)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Settings {
    pub id: i32,
    pub layout_type: String,
    pub color_mode: String,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = slides)]
pub struct UpdateSlide<'a> {
    pub id: &'a str,
    pub caption: Option<&'a str>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub active: Option<bool>,
    pub filetype: Option<&'a str>,
}
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: Option<String>,
    pub keplr_address: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

impl<'r> FromRow<'r, sqlx::postgres::PgRow> for User {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        Ok(User {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            password: row.try_get("password")?,
            keplr_address: row.try_get("keplr_address")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

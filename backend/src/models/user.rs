use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: Option<String>,
    pub keplr_address: Option<String>,
    pub pfp_url: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

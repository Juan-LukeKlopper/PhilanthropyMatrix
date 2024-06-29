use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub username: String,
    pub password: Option<String>,
    pub keplr_address: Option<String>,
}

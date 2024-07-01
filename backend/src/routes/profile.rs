use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use rocket::State;
use sqlx::prelude::FromRow;
use sqlx::PgPool;

use crate::claims::Claims;

#[derive(Serialize, Deserialize, FromRow)]
pub struct UserProfile {
    pub username: String,
    pub keplr_address: Option<String>,
}

#[get("/profile")]
pub async fn get_profile(user: Claims, pool: &State<PgPool>) -> Option<Json<UserProfile>> {
    let user: UserProfile = sqlx::query_as("SELECT * FROM users WHERE username = $1")
        .bind(&user.name)
        .fetch_one(pool.inner())
        .await
        .ok()?;

    Some(Json(UserProfile {
        username: user.username,
        keplr_address: user.keplr_address,
    }))
}

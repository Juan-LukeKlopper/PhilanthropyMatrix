use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use rocket::State;
use sqlx::PgPool;

use crate::claims::Claims;
use crate::models::user::User;

#[derive(Serialize, Deserialize)]
pub struct UserProfile {
    pub username: String,
    pub keplr_address: Option<String>,
}

#[get("/profile")]
pub async fn get_profile(user: Claims, pool: &State<PgPool>) -> Option<Json<UserProfile>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&user.name)
        .fetch_one(pool.inner())
        .await
        .ok()?;

    Some(Json(UserProfile {
        username: user.username,
        keplr_address: user.keplr_address,
    }))
}

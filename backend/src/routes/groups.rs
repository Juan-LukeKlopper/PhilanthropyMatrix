use rocket::serde::json::Json;
use rocket::{get, post, State};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use rocket::response::status::Custom;
use rocket::http::Status;

use crate::MyState;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Group {
    id: i32,
    name: String,
    image_url: Option<String>,
    cashout_wallet_address: String,
    primary_color: Option<String>,
    secondary_color: Option<String>,
    about_us: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateGroupRequest {
    name: String,
    image_url: Option<String>,
    cashout_wallet_address: String,
    primary_color: Option<String>,
    secondary_color: Option<String>,
    about_us: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct GroupListItem {
    id: i32,
    name: String,
}

// Endpoint to add a new group
#[post("/add", data = "<group>")]
pub async fn add_group(
    group: Json<CreateGroupRequest>,
    state: &State<MyState>,
) -> Result<Json<Group>, Custom<String>> {
    let new_group = sqlx::query_as::<_, Group>(
        "INSERT INTO groups (name, image_url, cashout_wallet_address, primary_color, secondary_color, about_us) 
        VALUES ($1, $2, $3, $4, $5, $6) 
        RETURNING id, name, image_url, cashout_wallet_address, primary_color, secondary_color, about_us"
    )
    .bind(&group.name)
    .bind(&group.image_url)
    .bind(&group.cashout_wallet_address)
    .bind(&group.primary_color)
    .bind(&group.secondary_color)
    .bind(&group.about_us)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(new_group))
}

// Endpoint to show all groups (name and id only)
#[get("/all")]
pub async fn show_all_groups(state: &State<MyState>) -> Result<Json<Vec<GroupListItem>>, Custom<String>> {
    let groups = sqlx::query_as::<_, GroupListItem>("SELECT id, name FROM groups")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(groups))
}

// Endpoint to get specific group info by id
#[get("/<id>")]
pub async fn get_group(id: i32, state: &State<MyState>) -> Result<Json<Group>, Custom<String>> {
    let group = sqlx::query_as::<_, Group>("SELECT * FROM groups WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(group))
}



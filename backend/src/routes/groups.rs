use rocket::serde::json::Json;
use rocket::{get, post, State};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use rocket::response::status::Custom;
use rocket::http::Status;

use crate::claims::Claims;
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

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct ProposedGroup {
    id: i32,
    name: String,
    user_id: Option<i32>,
    image_url: Option<String>,
    cashout_wallet_address: String,
    primary_color: Option<String>,
    secondary_color: Option<String>,
    about_us: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateProposedGroupRequest {
    name: String,
    image_url: Option<String>,
    cashout_wallet_address: String,
    primary_color: Option<String>,
    secondary_color: Option<String>,
    about_us: Option<String>,
}

#[derive(Deserialize)]
pub struct ApproveGroupRequest {
    group_id: i32,
}


#[derive(Serialize, Deserialize, FromRow)]
pub struct GroupListItem {
    id: i32,
    name: String,
}


// Endpoint to add a new group
#[post("/groups/proposal/add", data = "<group>")]
pub async fn add_proposed_group(
    group: Json<CreateProposedGroupRequest>,
    state: &State<MyState>,
    claims: Claims
) -> Result<Json<ProposedGroup>, Custom<String>> {

    let user_id = claims.id;
    let new_group = sqlx::query_as::<_, ProposedGroup>(
        "INSERT INTO group_proposals (name, user_id, image_url, cashout_wallet_address, primary_color, secondary_color, about_us) 
        VALUES ($1, $2, $3, $4, $5, $6, $7) 
        RETURNING id, name, user_id, image_url, cashout_wallet_address, primary_color, secondary_color, about_us"
    )
    .bind(&group.name)
    .bind(user_id)
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

// Endpoint to show all group proposals (name and id only)
#[get("/groups/proposals")]
pub async fn show_all_proposed_groups(state: &State<MyState>) -> Result<Json<Vec<GroupListItem>>, Custom<String>> {
    let groups = sqlx::query_as::<_, GroupListItem>("SELECT id, name FROM group_proposals")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(groups))
}

// Endpoint to get specific group proposal info by id
#[get("/groups/proposal/<id>")]
pub async fn get_group_proposal(id: i32, state: &State<MyState>) -> Result<Json<ProposedGroup>, Custom<String>> {
    let group = sqlx::query_as::<_, ProposedGroup>("SELECT * FROM group_proposals WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(group))
}

// Endpoint to approve a group proposal
#[post("/groups/approve", data = "<approve_request>")]
pub async fn approve_group(
    approve_request: Json<ApproveGroupRequest>,
    state: &State<MyState>,
    claims: Claims,
) -> Result<Json<Group>, Custom<String>> {

    if !claims.group_ids.contains(&1) {
        return Err(Custom(Status::Unauthorized, "You are not a system admin!".to_string()));
    }

    // Start a database transaction
    let mut tx = state.pool.begin().await.map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    // Fetch the proposed group details
    let proposed_group = sqlx::query_as::<_, ProposedGroup>("SELECT * FROM group_proposals WHERE id = $1")
        .bind(approve_request.group_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    // Insert the group into the groups table
    let group = sqlx::query_as::<_, Group>(
        "INSERT INTO groups (name, image_url, cashout_wallet_address, primary_color, secondary_color, about_us) 
        VALUES ($1, $2, $3, $4, $5, $6) 
        RETURNING id, name, image_url, cashout_wallet_address, primary_color, secondary_color, about_us"
    )
    .bind(&proposed_group.name)
    .bind(&proposed_group.image_url)
    .bind(&proposed_group.cashout_wallet_address)
    .bind(&proposed_group.primary_color)
    .bind(&proposed_group.secondary_color)
    .bind(&proposed_group.about_us)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    // Associate the user with the new group and set them as admin
    sqlx::query("INSERT INTO user_groups (user_id, group_id, is_admin) VALUES ($1, $2, $3)")
        .bind(&proposed_group.user_id)
        .bind(group.id)
        .bind(true)
        .execute(&mut *tx)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    // Remove the group from the proposals table
    sqlx::query("DELETE FROM group_proposals WHERE id = $1")
        .bind(approve_request.group_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    // Commit the transaction
    tx.commit().await.map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(group))
}

#[post("/groups/reject", data = "<reject_request>")]
pub async fn reject_group(
    reject_request: Json<ApproveGroupRequest>,
    state: &State<MyState>,
    claims: Claims,
) -> Result<Json<ProposedGroup>, Custom<String>> {

    let group_ids = claims.group_ids;

    if !group_ids.contains(&1) {
        return Err(Custom(Status::Unauthorized, "You are not a system admin!".to_string()));

    }

    let deleted_group = sqlx::query_as::<_, ProposedGroup>("DELETE FROM group_proposals WHERE id = $1 RETURNING *")
        .bind(reject_request.group_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(deleted_group))
}

#[post("/groups/remove", data = "<reject_request>")]
pub async fn remove_group(
    reject_request: Json<ApproveGroupRequest>,
    state: &State<MyState>,
    claims: Claims,
) -> Result<Json<Group>, Custom<String>> {
    let group_ids = claims.group_ids;

    if !group_ids.contains(&1) {
        return Err(Custom(Status::Unauthorized, "You are not a system admin!".to_string()));

    }

    let deleted_group = sqlx::query_as::<_, Group>("DELETE FROM groups WHERE id = $1 RETURNING *")
        .bind(reject_request.group_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(deleted_group))
}


// Endpoint to show all groups (name and id only)
#[get("/groups/all")]
pub async fn show_all_groups(state: &State<MyState>) -> Result<Json<Vec<GroupListItem>>, Custom<String>> {
    let groups = sqlx::query_as::<_, GroupListItem>("SELECT id, name FROM groups")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(groups))
}

// Endpoint to get specific group info by id
#[get("/groups/<id>")]
pub async fn get_group(id: i32, state: &State<MyState>) -> Result<Json<Group>, Custom<String>> {
    let group = sqlx::query_as::<_, Group>("SELECT * FROM groups WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(group))
}


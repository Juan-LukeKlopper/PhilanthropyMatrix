use rocket::serde::json::Json;
use rocket::{get, post, delete, State};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use rocket::response::status::Custom;
use rocket::http::Status;

use crate::claims::Claims;
use crate::MyState;

#[derive(Deserialize)]
pub struct AddMemberRequest {
    pub group_id: i32,
    pub user_id: i32,
    pub is_admin: bool,
}

#[derive(Deserialize)]
pub struct RemoveMemberRequest {
    pub group_id: i32,
    pub user_id: i32,
}

#[derive(Serialize, FromRow)]
pub struct GroupMember {
    pub user_id: i32,
    pub is_admin: bool,
}

// Endpoint to add a new member to a group
#[post("/groups/add_member", data = "<member_request>")]
pub async fn add_member(
    member_request: Json<AddMemberRequest>,
    state: &State<MyState>,
    claims: Claims,
) -> Result<Json<String>, Custom<String>> {
    let is_admin: bool = sqlx::query_scalar("SELECT is_admin FROM user_groups WHERE user_id = $1 AND group_id = $2")
        .bind(claims.id)
        .bind(member_request.group_id)
        .fetch_one(&state.pool)
        .await
        .unwrap_or(Some(false))
        .unwrap_or(false); 

    if !is_admin || !claims.group_ids.contains(&1) {
        return Err(Custom(Status::Unauthorized, "You are not an admin of this group".to_string()));
    }

    sqlx::query("INSERT INTO user_groups (user_id, group_id, is_admin) VALUES ($1, $2, $3)")
    .bind(member_request.user_id)
    .bind(member_request.group_id)
    .bind(member_request.is_admin)
    .execute(&state.pool)
    .await
    .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json("Member added successfully".to_string()))
}

// Endpoint to list all members of a group
#[get("/groups/members/<group_id>")]
pub async fn list_group_members(
    group_id: i32,
    state: &State<MyState>,
    claims: Claims,
) -> Result<Json<Vec<GroupMember>>, Custom<String>> {
    let is_admin: bool = sqlx::query_scalar("SELECT is_admin FROM user_groups WHERE user_id = $1 AND group_id = $2")
        .bind(claims.id)
        .bind(group_id)
        .fetch_optional(&state.pool)
        .await
        .unwrap_or(Some(false))
        .unwrap_or(false); 

        
    if !is_admin || !claims.group_ids.contains(&1) {
        return Err(Custom(Status::Unauthorized, "You are not an admin of this group or system".to_string()));
    }

    let members = sqlx::query_as::<_, GroupMember>("SELECT user_id, is_admin FROM user_groups WHERE group_id = $1")
        .bind(group_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(members))
}

// Endpoint to remove a member from a group
#[delete("/groups/remove_member", data = "<remove_request>")]
pub async fn remove_member(
    remove_request: Json<RemoveMemberRequest>,
    state: &State<MyState>,
    claims: Claims,
) -> Result<Json<String>, Custom<String>> {

    let is_admin: bool = sqlx::query_scalar("SELECT is_admin FROM user_groups WHERE user_id = $1 AND group_id = $2")
    .bind(claims.id)
    .bind(remove_request.group_id)
    .fetch_optional(&state.pool)
    .await
    .unwrap_or(Some(false))  // Return Some(false) in case of an error or if no value is found
    .unwrap_or(false);  // Unwrap the Option to get a bool
        
    if !is_admin || !claims.group_ids.contains(&1) {
        return Err(Custom(Status::Unauthorized, "You are not an admin of this group".to_string()));
    }

    sqlx::query("DELETE FROM user_groups WHERE user_id = $1 AND group_id = $2")
        .bind(remove_request.user_id)
        .bind(remove_request.group_id)
        .execute(&state.pool)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json("Member removed successfully".to_string()))
}

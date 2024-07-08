use rocket::serde::json::Json;
use rocket::{get, post, State};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use rocket::response::status::Custom;
use rocket::http::Status;

use crate::claims::Claims;
use crate::MyState;

#[derive(Serialize, Deserialize, FromRow)]
pub struct DonationProposal {
    id: i32,
    group_id: i32,
    name: String,
    symbol: String,
    cost: i32,
    description: String,
    image_url: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Donation {
    id: i32,
    group_id: i32,
    name: String,
    symbol: String,
    cost: i32,
    contract_address: Option<String>,
    description: String,
    image_url: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateDonationProposalRequest {
    group_id: i32,
    name: String,
    symbol: String,
    cost: i32,
    description: String,
    image_url: Option<String>,
}

#[derive(Deserialize)]
pub struct ApproveDonationRequest {
    proposal_id: i32,
    group_id: i32,
}

#[derive(Deserialize)]
pub struct LinkSecretAddressRequest {
    donation_id: i32,
    group_id: i32,
    contract_address: String,
}

#[post("/donations/link_address", data = "<request>")]
pub async fn link_secret_network_address(
    request: Json<LinkSecretAddressRequest>,
    state: &State<MyState>,
    claims: Claims,
) -> Result<Custom<String>, Custom<String>> {
    let is_admin: bool = sqlx::query_scalar("SELECT is_admin FROM user_groups WHERE user_id = $1 AND group_id = $2")
        .bind(claims.id)
        .bind(request.group_id)
        .fetch_one(&state.pool)
        .await
        .unwrap_or(Some(false))
        .unwrap_or(false); 

    if !is_admin && !claims.group_ids.contains(&1) {
        return Err(Custom(Status::Unauthorized, "You are not an admin of this group or system".to_string()));
    }

    sqlx::query(
        "UPDATE donations SET contract_address = $1 WHERE id = $2"
    )
    .bind(&request.contract_address)
    .bind(request.donation_id)
    .execute(&state.pool)
    .await
    .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Custom(Status::Ok, "Secret Network address linked to donation".to_string()))
}


#[post("/donations/proposal/add", data = "<proposal>")]
pub async fn add_donation_proposal(
    proposal: Json<CreateDonationProposalRequest>,
    state: &State<MyState>,
    claims: Claims,
) -> Result<Json<DonationProposal>, Custom<String>> {
    let is_member_or_admin = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM user_groups WHERE user_id = $1 AND group_id = $2)"
    )
    .bind(claims.id)
    .bind(proposal.group_id)
    .fetch_one(&state.pool)
    .await
    .unwrap_or(false);

    if !is_member_or_admin && claims.group_ids.contains(&1) {
        return Err(Custom(Status::Unauthorized, "You are not authorized to propose a donation for this group".to_string()));
    }

    let new_proposal = sqlx::query_as::<_, DonationProposal>(
        "INSERT INTO donation_proposals (group_id, name, symbol, cost, description, image_url) 
        VALUES ($1, $2, $3, $4, $5, $6) 
        RETURNING id, group_id, name, symbol, cost, description, image_url"
    )
    .bind(proposal.group_id)
    .bind(&proposal.name)
    .bind(&proposal.symbol)
    .bind(proposal.cost)
    .bind(&proposal.description)
    .bind(&proposal.image_url)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(new_proposal))
}

#[post("/donations/approve", data = "<approve_request>")]
pub async fn approve_donation(
    approve_request: Json<ApproveDonationRequest>,
    state: &State<MyState>,
    claims: Claims,
) -> Result<Json<Donation>, Custom<String>> {
    
    let proposal = sqlx::query_as::<_, DonationProposal>("SELECT * FROM donation_proposals WHERE id = $1")
        .bind(approve_request.proposal_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    let is_admin = sqlx::query_scalar::<_, bool>(
        "SELECT is_admin FROM user_groups WHERE user_id = $1 AND group_id = $2"
    )
    .bind(claims.id)
    .bind(proposal.group_id)
    .fetch_optional(&state.pool)
    .await
    .unwrap_or(Some(false))
    .unwrap_or(false);

    if !is_admin && claims.group_ids.contains(&1) {
        return Err(Custom(Status::Unauthorized, "You are not authorized to approve this donation".to_string()));
    }

    let mut tx = state.pool.begin().await.map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    let donation = sqlx::query_as::<_, Donation>(
        "INSERT INTO donations (group_id, name, symbol, cost, description, image_url) 
        VALUES ($1, $2, $3, $4, $5, $6) 
        RETURNING id, group_id, name, symbol, cost, contract_address, description, image_url"
    )
    .bind(proposal.group_id)
    .bind(&proposal.name)
    .bind(&proposal.symbol)
    .bind(proposal.cost)
    .bind(&proposal.description)
    .bind(&proposal.image_url)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    sqlx::query("INSERT INTO group_donations (group_id, donation_id) VALUES ($1, $2)")
        .bind(proposal.group_id)
        .bind(donation.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    sqlx::query("DELETE FROM donation_proposals WHERE id = $1")
        .bind(approve_request.proposal_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    tx.commit().await.map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(donation))
}

#[get("/donations/list/<group_id>")]
pub async fn list_donations_for_group(group_id: i32, state: &State<MyState>) -> Result<Json<Vec<Donation>>, Custom<String>> {
    let donations = sqlx::query_as::<_, Donation>("SELECT * FROM donations WHERE group_id = $1")
        .bind(group_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(donations))
}

#[get("/donations/<group_id>/list/proposals")]
pub async fn list_donation_proposals_for_group(group_id: i32, state: &State<MyState>) -> Result<Json<Vec<DonationProposal>>, Custom<String>> {
    let donations = sqlx::query_as::<_, DonationProposal>("SELECT * FROM donation_proposals WHERE group_id = $1")
        .bind(group_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(donations))
}

#[get("/donations/proposal/<id>")]
pub async fn get_donation_proposal(id: i32, state: &State<MyState>) -> Result<Json<DonationProposal>, Custom<String>> {
    let donation_proposal = sqlx::query_as::<_, DonationProposal>("SELECT * FROM donation_proposals WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(donation_proposal))
}


#[get("/donations/<id>")]
pub async fn get_donation(id: i32, state: &State<MyState>) -> Result<Json<Donation>, Custom<String>> {
    let donation = sqlx::query_as::<_, Donation>("SELECT * FROM donations WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(donation))
}


#[get("/donations/active")]
pub async fn get_active_donation(state: &State<MyState>) -> Result<Json<Vec<Donation>>, Custom<String>> {
    let donations = sqlx::query_as::<_, Donation>("SELECT * FROM donations WHERE contract_address IS NOT NULL")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(donations))
}


#[post("/donations/reject", data = "<reject_request>")]
pub async fn reject_donation_proposal(
    reject_request: Json<ApproveDonationRequest>,
    state: &State<MyState>,
    claims: Claims,
) -> Result<Json<DonationProposal>, Custom<String>> {

    let is_admin: bool = sqlx::query_scalar("SELECT is_admin FROM user_groups WHERE user_id = $1 AND group_id = $2")
        .bind(claims.id)
        .bind(reject_request.group_id)
        .fetch_one(&state.pool)
        .await
        .unwrap_or(Some(false))
        .unwrap_or(false); 

    if !is_admin && !claims.group_ids.contains(&1) {
        return Err(Custom(Status::Unauthorized, "You are not an admin of this group or system".to_string()));
    }

    let deleted_group = sqlx::query_as::<_, DonationProposal>("DELETE FROM donation_proposals WHERE id = $1 RETURNING *")
        .bind(reject_request.proposal_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(deleted_group))
}

#[post("/donations/remove", data = "<remove_request>")]
pub async fn remove_donation(
    remove_request: Json<ApproveDonationRequest>,
    state: &State<MyState>,
    claims: Claims,
) -> Result<Json<Donation>, Custom<String>> {

    let is_admin: bool = sqlx::query_scalar("SELECT is_admin FROM user_groups WHERE user_id = $1 AND group_id = $2")
        .bind(claims.id)
        .bind(remove_request.group_id)
        .fetch_one(&state.pool)
        .await
        .unwrap_or(Some(false))
        .unwrap_or(false); 

    if !is_admin && !claims.group_ids.contains(&1) {
        return Err(Custom(Status::Unauthorized, "You are not an admin of this group or system".to_string()));
    }

    let deleted_donation = sqlx::query_as::<_, Donation>("DELETE FROM donations WHERE id = $1 RETURNING *")
        .bind(remove_request.proposal_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(deleted_donation))
}
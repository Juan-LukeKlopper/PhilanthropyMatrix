#[macro_use]
extern crate rocket;

use rocket_cors::{AllowedOrigins, CorsOptions};
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};

mod claims;
mod models;
mod routes;

use routes::{auth, donations, group_members, groups, profile};

pub struct MyState {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn rocket(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allow_credentials(true)
        .to_cors()
        .expect("error creating CORS options");

    let state = MyState { pool: pool.clone() };

    let rocket = rocket::build()
        .manage(state)
        .mount("/", routes![
            auth::login,
            auth::register,
            auth::keplr_login,
            auth::change_credentials,
            auth::add_wallet,
            profile::get_profile,
            groups::add_proposed_group,
            groups::show_all_proposed_groups,
            groups::get_group_proposal,
            groups::approve_group,
            groups::reject_group,
            groups::remove_group,
            groups::show_all_groups, 
            groups::get_group,
            group_members::add_member,
            group_members::list_group_members,
            group_members::remove_member,
            donations::add_donation_proposal,
            donations::approve_donation,
            donations::list_donations_for_group,
            donations::get_donation,
            donations::reject_donation_proposal,
            donations::remove_donation,
            donations::list_donation_proposals_for_group,
            donations::get_donation_proposal,
            donations::link_secret_network_address,
            donations::get_active_donation
        ])
        .manage(pool)
        .attach(cors);

    Ok(rocket.into())
}

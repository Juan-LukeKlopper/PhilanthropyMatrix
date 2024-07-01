#[macro_use]
extern crate rocket;

use rocket_cors::{AllowedOrigins, CorsOptions};
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};

mod claims;
mod models;
mod routes;

use routes::{auth, profile, groups};

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
            groups::add_group, 
            groups::show_all_groups, 
            groups::get_group
        ])
        .manage(pool)
        .attach(cors);

    Ok(rocket.into())
}

use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use rocket::State;
use verify_keplr_sign::{verify_arbitrary, Signature, PublicKey};
use sqlx::PgPool;

use crate::claims::Claims;
use crate::models::user::User;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct KeplrLoginRequest {
    pub address: String,
    pub pubkey: String,
    pub sign_message: String,
    pub signature: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LinkAccountsRequest {
    pub username: String,
    pub password: String,
    pub keplr_address: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize)]
pub struct PublicResponse {
    message: String,
}

#[post("/login", data = "<login>")]
pub async fn login(login: Json<LoginRequest>, pool: &State<PgPool>) -> Result<Json<LoginResponse>, Custom<String>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&login.username)
        .fetch_one(pool.inner())
        .await;

    match user {
        Ok(u) => {
            if u.password.as_ref() == Some(&login.password) {
                let claim = Claims::from_name(&login.username);
                let response = LoginResponse {
                    token: claim.into_token()?,
                };
                Ok(Json(response))
            } else {
                Err(Custom(Status::Unauthorized, "Invalid username or password".to_string()))
            }
        }
        Err(_) => Err(Custom(Status::Unauthorized, "Invalid username or password".to_string())),
    }
}

#[post("/register", data = "<register>")]
pub async fn register(register: Json<RegisterRequest>, pool: &State<PgPool>) -> Result<Json<PublicResponse>, Custom<String>> {
    let result = sqlx::query("INSERT INTO users (username, password, keplr_address) VALUES ($1, $2, $3)")
        .bind(&register.username)
        .bind(&register.password)
        .bind(Option::<String>::None)
        .execute(pool.inner())
        .await;

    match result {
        Ok(_) => Ok(Json(PublicResponse {
            message: "User registered successfully".to_string(),
        })),
        Err(_) => Err(Custom(Status::Conflict, "Username already taken".to_string())),
    }
}

#[post("/keplr-login", data = "<keplr_login>")]
pub async fn keplr_login(keplr_login: Json<KeplrLoginRequest>, pool: &State<PgPool>) -> Result<Json<LoginResponse>, Custom<String>> {
    let address = &keplr_login.address;
    let pubkey = &keplr_login.pubkey;
    let sign_message = keplr_login.sign_message.as_bytes();
    let signature = &keplr_login.signature;

    let public_key = PublicKey {
        sig_type: "tendermint/PubKeySecp256k1".to_string(),
        sig_value: pubkey.clone(),
    };

    let sig = Signature {
        pub_key: public_key,
        signature: signature.clone(),
    };

    if !verify_arbitrary(address, &keplr_login.pubkey, sign_message, &sig) {
        return Err(Custom(Status::Unauthorized, "Invalid signature".to_string()));
    }

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE keplr_address = $1 OR username = $2")
        .bind(address)
        .bind(address)
        .fetch_optional(pool.inner())
        .await;

    match user {
        Ok(Some(u)) => {
            let claim = Claims::from_name(&u.username);
            let response = LoginResponse {
                token: claim.into_token()?,
            };
            Ok(Json(response))
        }
        Ok(None) => {
            sqlx::query("INSERT INTO users (username, keplr_address) VALUES ($1, $2)")
                .bind(address)
                .bind(address)
                .execute(pool.inner())
                .await
                .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

            let claim = Claims::from_name(address);
            let response = LoginResponse {
                token: claim.into_token()?,
            };
            Ok(Json(response))
        }
        Err(e) => Err(Custom(Status::InternalServerError, e.to_string())),
    }
}

#[post("/link-accounts", data = "<link_request>")]
pub async fn link_accounts(link_request: Json<LinkAccountsRequest>, pool: &State<PgPool>) -> Result<Json<PublicResponse>, Custom<String>> {
    let web2_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&link_request.username)
        .fetch_one(pool.inner())
        .await;

    let mut web2_user = match web2_user {
        Ok(user) => user,
        Err(_) => return Err(Custom(Status::NotFound, "No account found for this username".to_string())),
    };

    if web2_user.password.as_ref() != Some(&link_request.password) {
        return Err(Custom(Status::Unauthorized, "Invalid password".to_string()));
    }

    let web3_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE keplr_address = $1")
        .bind(&link_request.keplr_address)
        .fetch_optional(pool.inner())
        .await;

    match web3_user {
        Ok(Some(web3_user)) => {
            web2_user.keplr_address = web3_user.keplr_address.clone();
            sqlx::query("UPDATE users SET keplr_address = $1 WHERE username = $2")
                .bind(&web2_user.keplr_address)
                .bind(&web2_user.username)
                .execute(pool.inner())
                .await
                .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

            sqlx::query("DELETE FROM users WHERE username = $1")
                .bind(&web3_user.username)
                .execute(pool.inner())
                .await
                .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
        }
        Ok(None) => {
            web2_user.keplr_address = Some(link_request.keplr_address.clone());
            sqlx::query("UPDATE users SET keplr_address = $1 WHERE username = $2")
                .bind(&web2_user.keplr_address)
                .bind(&web2_user.username)
                .execute(pool.inner())
                .await
                .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
        }
        Err(e) => return Err(Custom(Status::InternalServerError, e.to_string())),
    }

    Ok(Json(PublicResponse {
        message: "Accounts linked successfully".to_string(),
    }))
}

use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
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
pub struct ChangeCredentialsRequest {
    pub keplr_address: String,
    pub new_username: String,
    pub new_password: String,
}

#[derive(Deserialize)]
pub struct AddWalletRequest {
    pub keplr_address: String,
    pub pubkey: String,
    pub sign_message: String,
    pub signature: String,
}


#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: i32,
    pub groups: Vec<i32>
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
                let groups: Vec<i32> = sqlx::query_as::<_, (i32,)>("SELECT group_id FROM user_groups WHERE user_id = $1")
                    .bind(u.id)
                    .fetch_all(pool.inner())
                    .await
                    .map(|result| {
                        if result.is_empty() {
                            vec![0]
                        } else {
                            result.into_iter().map(|group: (i32,)| group.0).collect()
                        }
                    })
                    .unwrap_or_else(|_| vec![0]);  // Default to vec![0] in case of any error

                let claim = Claims::new(u.id, &u.username, groups.clone());
                let response = LoginResponse {
                    token: claim.into_token()?,
                    user_id: u.id,
                    groups: groups,
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
            let groups: Vec<i32> = sqlx::query_as::<_, (i32,)>("SELECT group_id FROM user_groups WHERE user_id = $1")
                .bind(u.id)
                .fetch_all(pool.inner())
                .await
                .map(|result| {
                    if result.is_empty() {
                        vec![0]
                    } else {
                        result.into_iter().map(|group: (i32,)| group.0).collect()
                    }
                })
                .unwrap_or_else(|_| vec![0]);  // Default to vec![0] in case of any error

            let claim = Claims::new(u.id, &u.username, groups.clone());
            let response = LoginResponse {
                token: claim.into_token()?,
                user_id: u.id,
                groups: groups
            };
            Ok(Json(response))
        }
        Ok(None) => {
            // Insert new user and get the new user ID
            let row: (i32,) = sqlx::query_as("INSERT INTO users (username, keplr_address) VALUES ($1, $2) RETURNING id")
                .bind(address)
                .bind(address)
                .fetch_one(pool.inner())
                .await
                .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
                
            let new_user_id = row.0;

            let claim = Claims::new(new_user_id, address, vec![0]);
            let response = LoginResponse {
                token: claim.into_token()?,
                user_id: new_user_id,
                groups: vec![0]
            };
            Ok(Json(response))
        }
        Err(e) => Err(Custom(Status::InternalServerError, e.to_string())),
    }
}



#[post("/add-wallet", data = "<request>")]
pub async fn add_wallet(request: Json<AddWalletRequest>,user: Claims, pool: &State<PgPool>) -> Result<Json<PublicResponse>, Custom<String>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&user.name)
        .fetch_optional(pool.inner())
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    match user {
        Some(mut user) => {
            let keplr_address_exists = sqlx::query("SELECT 1 FROM users WHERE keplr_address = $1")
                .bind(&request.keplr_address)
                .fetch_optional(pool.inner())
                .await
                .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?
                .is_some();

            if keplr_address_exists {
                return Err(Custom(Status::Conflict, "Keplr address already linked to another account".to_string()));
            }

            let public_key = PublicKey {
                sig_type: "tendermint/PubKeySecp256k1".to_string(),
                sig_value: request.pubkey.clone(),
            };

            let sig = Signature {
                pub_key: public_key,
                signature: request.signature.clone(),
            };

            if !verify_arbitrary(&request.keplr_address, &request.pubkey, request.sign_message.as_bytes(), &sig) {
                return Err(Custom(Status::Unauthorized, "Invalid signature".to_string()));
            }

            user.keplr_address = Some(request.keplr_address.clone());

            sqlx::query("UPDATE users SET keplr_address = $1 WHERE username = $2")
                .bind(&user.keplr_address)
                .bind(&user.username)
                .execute(pool.inner())
                .await
                .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

            Ok(Json(PublicResponse {
                message: "Wallet address added successfully".to_string(),
            }))
        }
        None => Err(Custom(Status::NotFound, "User not found".to_string())),
    }
}


#[post("/change-credentials", data = "<request>")]
pub async fn change_credentials(request: Json<ChangeCredentialsRequest>, __claims: Claims,pool: &State<PgPool>) -> Result<Json<PublicResponse>, Custom<String>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE keplr_address = $1")
        .bind(&request.keplr_address)
        .fetch_optional(pool.inner())
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    match user {
        Some(mut user) => {
            let username_exists = sqlx::query("SELECT 1 FROM users WHERE username = $1")
                .bind(&request.new_username)
                .fetch_optional(pool.inner())
                .await
                .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?
                .is_some();

            if username_exists {
                return Err(Custom(Status::Conflict, "Username already taken".to_string()));
            }

            user.username = request.new_username.clone();
            user.password = Some(request.new_password.clone());

            sqlx::query("UPDATE users SET username = $1, password = $2 WHERE keplr_address = $3")
                .bind(&user.username)
                .bind(&user.password)
                .bind(&request.keplr_address)
                .execute(pool.inner())
                .await
                .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

            Ok(Json(PublicResponse {
                message: "Username and password updated successfully".to_string(),
            }))
        }
        None => Err(Custom(Status::NotFound, "User not found".to_string())),
    }
}
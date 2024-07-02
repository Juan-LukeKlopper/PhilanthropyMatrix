use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use rocket::{http::Status, request::{FromRequest, Outcome}, response::status::Custom};
use serde::{Deserialize, Serialize};

const BEARER: &str = "Bearer ";
const AUTHORIZATION: &str = "Authorization";
const SECRET: &str = "secret";

lazy_static! {
    static ref TOKEN_EXPIRATION: Duration = Duration::minutes(60);
}

#[derive(Debug, PartialEq)]
pub enum AuthenticationError {
    Missing,
    Decoding(String),
    Expired,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub id: i32,
    pub name: String,
    pub group_ids: Vec<i32>,
    exp: usize,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = AuthenticationError;

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one(AUTHORIZATION) {
            None => Outcome::Error((Status::Forbidden, AuthenticationError::Missing)),
            Some(value) => match Claims::from_authorization(value) {
                Err(e) => Outcome::Error((Status::Forbidden, e)),
                Ok(claims) => Outcome::Success(claims),
            },
        }
    }
}

impl Claims {
    pub fn new(id: i32, name: &str, group_ids: Vec<i32>) -> Self {
        Self {
            id,
            name: name.to_string(),
            group_ids,
            exp: 0,
        }
    }

    fn from_authorization(value: &str) -> Result<Self, AuthenticationError> {
        let token = value.strip_prefix(BEARER).map(str::trim).ok_or(AuthenticationError::Missing)?;
        let token = decode::<Claims>(token, &DecodingKey::from_secret(SECRET.as_ref()), &Validation::default())
            .map_err(|e| match e.kind() {
                ErrorKind::ExpiredSignature => AuthenticationError::Expired,
                _ => AuthenticationError::Decoding(e.to_string()),
            })?;
        Ok(token.claims)
    }

    pub fn into_token(mut self) -> Result<String, Custom<String>> {
        let expiration = Utc::now().checked_add_signed(*TOKEN_EXPIRATION).expect("valid timestamp").timestamp();
        self.exp = expiration as usize;
        let token = encode(&Header::default(), &self, &EncodingKey::from_secret(SECRET.as_ref()))
            .map_err(|e| Custom(Status::BadRequest, e.to_string()))?;
        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_bearer() {
        let claim_err = Claims::from_authorization("no-Bearer-prefix").unwrap_err();
        assert_eq!(claim_err, AuthenticationError::Missing);
    }

    #[test]
    fn to_token_and_back() {
        let claim = Claims::new(1, "test runner",vec![1]);
        let token = claim.into_token().unwrap();
        let token = format!("Bearer {}", token);
        let claim = Claims::from_authorization(&token).unwrap();
        assert_eq!(claim.name, "test runner");
        assert_eq!(claim.group_ids, vec![1]);
    }
}

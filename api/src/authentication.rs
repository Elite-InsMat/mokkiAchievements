use crate::{AppState, JWT_SECRET_KEY_NAME};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation,
};
use lazy_static::lazy_static;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    response::status::Custom,
};
use serde::{Deserialize, Serialize};

const BEARER: &str = "Bearer ";
const AUTHORIZATION: &str = "Authorization";

lazy_static! {
    static ref TOKEN_EXPIRATION: Duration = Duration::days(4);
}

// Used when decoding a token to `Claims`
#[derive(Debug, PartialEq, Eq)]
pub enum AuthenticationError {
    Missing,
    Decoding(String),
    Expired,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub name: String,
    exp: usize,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = AuthenticationError;

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let state = request.rocket().state::<AppState>().unwrap();
        match request.headers().get_one(AUTHORIZATION) {
            None => Outcome::Failure((Status::Forbidden, AuthenticationError::Missing)),
            Some(value) => {
                match Claims::from_authorization(value, &state.1.get(JWT_SECRET_KEY_NAME).unwrap())
                {
                    Err(e) => Outcome::Failure((Status::Forbidden, e)),
                    Ok(claims) => Outcome::Success(claims),
                }
            }
        }
    }
}

impl Claims {
    pub fn from_name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            exp: 0,
        }
    }

    fn from_authorization(value: &str, secret: &str) -> Result<Self, AuthenticationError> {
        let token = value.strip_prefix(BEARER).map(str::trim);

        if token.is_none() {
            return Err(AuthenticationError::Missing);
        }

        let token = token.unwrap();

        let token = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| match e.kind() {
            ErrorKind::ExpiredSignature => AuthenticationError::Expired,
            _ => AuthenticationError::Decoding(e.to_string()),
        })?;

        Ok(token.claims)
    }

    pub fn into_token(mut self, secret: &str) -> Result<String, Custom<String>> {
        let expiration = Utc::now()
            .checked_add_signed(*TOKEN_EXPIRATION)
            .expect("failed to create an expiration time")
            .timestamp();

        self.exp = expiration as usize;

        let token = encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| Custom(Status::BadRequest, e.to_string()))?;

        Ok(token)
    }
}

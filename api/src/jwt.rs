use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use entity::async_graphql::Error;

pub enum TokenType {
    Access,
    Refresh,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32,
    scope: i32,
    exp: i64,
    token_type: i32,
}

const JWT_SECRET: &str = "secret";

pub async fn create_access_token(id: i32, scope: i32, token_type: TokenType) -> String {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: id,
        scope,
        exp: expiration,
        token_type: token_type as i32,
    };
    let header = Header::new(Algorithm::HS512);
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
    .map_err(|_| Error::new("Failed to create access token"))
    .unwrap()
}

pub async fn validate_token(token: &str) -> Result<Claims, Error> {
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|_| Error::new("Failed to decode token"))?;

    Ok(decoded.claims)
}

use anyhow::Context;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use uuid::Uuid;

use crate::jwt::claims::{Claims, TokenPair, TokenType};

const ACCESS_TOKEN_EXPIRATION_MINUTES: i64 = 15;
const REFRESH_TOKEN_EXPIRATION_DAYS: i64 = 120;

pub async fn encode_access_token(id: Uuid, secret: &str) -> anyhow::Result<String> {
    let exp =
        (Utc::now() + Duration::minutes(ACCESS_TOKEN_EXPIRATION_MINUTES)).timestamp() as usize;
    let claims = Claims::new(id, exp, TokenType::Access);
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    );
    token.context("Error encoding access token")
}

pub async fn encode_refresh_token(id: Uuid, secret: &str) -> anyhow::Result<String> {
    let exp = (Utc::now() + Duration::days(REFRESH_TOKEN_EXPIRATION_DAYS)).timestamp() as usize;
    let claims = Claims::new(id, exp, TokenType::Refresh);
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    );
    token.context("Error encoding refresh token")
}

pub async fn encode_token_pair(id: Uuid, secret: &str) -> anyhow::Result<TokenPair> {
    let access_token = encode_access_token(id, secret).await?;
    let refresh_token = encode_refresh_token(id, secret).await?;

    Ok(TokenPair {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: ACCESS_TOKEN_EXPIRATION_MINUTES * 60,
    })
}

pub fn validate_access_token(token: &str, secret: &str) -> anyhow::Result<Uuid> {
    let decode = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )?;

    if decode.claims.token_type != TokenType::Access {
        return Err(anyhow::anyhow!("Invalid token type"));
    }

    Ok(decode.claims.id)
}

pub async fn validate_refresh_token(token: &str, secret: &str) -> anyhow::Result<Uuid> {
    let decode = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )?;

    if decode.claims.token_type != TokenType::Refresh {
        return Err(anyhow::anyhow!("Invalid token type"));
    }

    Ok(decode.claims.id)
}

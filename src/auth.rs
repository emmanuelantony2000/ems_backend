use chrono::{Duration, DurationRound, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
}

impl Claims {
    pub fn new() -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(1);

        let exp = exp.timestamp() as usize;
        let iat = iat.timestamp() as usize;

        Self { exp, iat }
    }

    pub fn new_permanent() -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::weeks(52);

        let exp = exp.timestamp() as usize;
        let iat = iat.timestamp() as usize;

        Self { exp, iat }
    }
}

pub fn encode(claims: &Claims, secret: String) -> anyhow::Result<String> {
    Ok(jsonwebtoken::encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?)
}

pub fn decode(token: String, secret: String) -> anyhow::Result<Claims> {
    Ok(jsonwebtoken::decode(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?
    .claims)
}

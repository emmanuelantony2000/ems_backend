use std::fmt;
use std::str::FromStr;

use chrono::{Duration, Utc};
use data_encoding::HEXUPPER;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use ring::digest;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const BEARER: &str = "Bearer ";
pub const JWT_SECRET: &[u8] = b"This is a very big secret.";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
    pub role: String,
    pub sub: String,
}

impl Claims {
    pub fn new(sub: String, role: String) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(1);

        let exp = exp.timestamp() as usize;
        let iat = iat.timestamp() as usize;

        Self {
            exp,
            iat,
            role,
            sub,
        }
    }

    pub fn new_permanent(sub: String, role: String) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::weeks(52);

        let exp = exp.timestamp() as usize;
        let iat = iat.timestamp() as usize;

        Self {
            exp,
            iat,
            role,
            sub,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Role {
    Admin,
    User,
}

impl FromStr for Role {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Admin" => Ok(Self::Admin),
            "User" => Ok(Self::User),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Admin => write!(f, "Admin"),
            Self::User => write!(f, "User"),
        }
    }
}

pub fn encode(
    claims: &Claims,
    secret: impl AsRef<[u8]>,
) -> Result<String, jsonwebtoken::errors::Error> {
    Ok(jsonwebtoken::encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?)
}

pub fn decode(
    token: String,
    secret: impl AsRef<[u8]>,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    Ok(jsonwebtoken::decode(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?
    .claims)
}

pub fn create_jwt(id: Uuid, role: Role) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(id.to_string(), role.to_string());
    encode(&claims, JWT_SECRET)
}

pub fn generate_password(password: impl AsRef<str>, id: &Uuid) -> String {
    HEXUPPER.encode(
        digest::digest(
            &digest::SHA256,
            format!("{}{}", id, password.as_ref()).as_bytes(),
        )
        .as_ref(),
    )
}

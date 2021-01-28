use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub(super) email: String,
    pub(super) password: String,
    pub(super) permanent: Option<bool>,
}

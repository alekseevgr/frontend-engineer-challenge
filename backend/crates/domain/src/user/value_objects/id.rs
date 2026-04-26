use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct UserId(Uuid);

#[derive(Debug, thiserror::Error)]
#[error("Invalid user ID format")]
pub struct InvalidUserId;

impl UserId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_string(s: &str) -> Result<Self, InvalidUserId> {
        let uuid = Uuid::parse_str(s).map_err(|_| InvalidUserId)?;
        Ok(Self(uuid))
    }

    pub fn uuid(&self) -> Uuid {
        self.0
    }
}

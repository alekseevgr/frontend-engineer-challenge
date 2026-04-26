use async_trait::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub trait Hasher {
    fn hash(&self, value: &str) -> Result<String, HashError>;
    fn verify(&self, value: &str, hash: &str) -> Result<bool, HashError>;
}

#[derive(Debug, thiserror::Error)]
pub enum HashError {
    #[error("Hash creation error: {0}")]
    Creation(String),
    #[error("Hash verifying error: {0}")]
    Verifying(String),
}

use application::ports::hasher::{HashError, Hasher};
use async_trait::async_trait;
use bcrypt::{DEFAULT_COST, hash, verify};

pub struct BcryptHasher {
    cost: u32,
}

impl Default for BcryptHasher {
    fn default() -> Self {
        Self { cost: DEFAULT_COST }
    }
}

#[async_trait]
impl Hasher for BcryptHasher {
    fn hash(&self, value: &str) -> Result<String, HashError> {
        hash(value, self.cost).map_err(|e| HashError::Creation(e.to_string()))
    }

    fn verify(&self, value: &str, hash: &str) -> Result<bool, HashError> {
        verify(value, hash).map_err(|e| HashError::Verifying(e.to_string()))
    }
}

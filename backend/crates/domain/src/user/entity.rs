use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::{
    shared::value_objects::password_hash::PasswordHash,
    user::value_objects::{email::Email, id::UserId},
};

#[derive(Debug, Serialize)]
pub struct User {
    id: UserId,
    email: Email,
    password_hash: PasswordHash,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        id: UserId,
        email: Email,
        password_hash: PasswordHash,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            email,
            password_hash,
            created_at,
            updated_at,
        }
    }

    pub fn create(email: Email, password_hash: PasswordHash) -> Self {
        let now = Utc::now();
        Self {
            id: UserId::generate(),
            email,
            password_hash,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn email(&self) -> &Email {
        &self.email
    }
}

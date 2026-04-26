#[derive(Debug)]
pub struct RegisterUserRequest {
    pub email: String,
    pub password: String,
}

impl RegisterUserRequest {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}

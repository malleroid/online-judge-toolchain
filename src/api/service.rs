use reqwest::Client;
use std::error::Error;

pub struct LoginResult {
    pub success: bool,
    pub message: Option<String>,
}

pub trait ServiceLogin {
    async fn login(
        &self,
        client: &Client,
        username: &str,
        password: &str,
    ) -> Result<LoginResult, Box<dyn Error>>;
}

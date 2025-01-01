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

pub trait ServiceDownload {
    async fn download(
        &self,
        client: &Client,
        contest_id: &str,
        output_dir: Option<&str>,
    ) -> Result<(), Box<dyn Error>>;
}

use crate::api::{atcoder::AtCoder, service::ServiceLogin};
use reqwest::Client;
use rpassword::prompt_password;
use std::error::Error;

pub async fn login_command(
    service_name: &str,
    username: &str,
    password: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let password = match password {
        Some(password) => password.to_string(),
        None => prompt_password("Password: ").unwrap(),
    };

    let client = Client::builder().cookie_store(true).build()?;

    match service_name {
        "atcoder" => {
            let atcoder = AtCoder;
            let result = atcoder.login(&client, username, &password).await?;
            if result.success {
                println!(
                    "{}",
                    result.message.unwrap_or("Login successful!".to_string())
                );
            } else {
                println!("{}", result.message.unwrap_or("Login failed.".to_string()));
            }
        }
        _ => {
            println!("Unsupported service: {}", service_name);
        }
    }

    Ok(())
}

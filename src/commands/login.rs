use crate::api::{atcoder::AtCoder, service::ServiceLogin};
use crate::session::SessionManager;
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

    let session_manager = SessionManager::new();
    let client = session_manager.create_client()?;

    match service_name {
        "atcoder" => {
            let atcoder = AtCoder;
            let result = atcoder.login(&client, username, &password).await?;
            if result.success {
                println!(
                    "{}",
                    result.message.unwrap_or("Login successful!".to_string())
                );
                session_manager.save_service_session(service_name)?;
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

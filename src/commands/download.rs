use crate::api::{atcoder::AtCoder, service::ServiceDownload};
use crate::session::SessionManager;
use std::error::Error;

pub async fn download_command(
    service_name: &str,
    contest_id: &str,
    output_dir: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let session_manager = SessionManager::new();
    let client = session_manager.create_client()?;

    match service_name {
        "atcoder" => {
            let atcoder = AtCoder;
            atcoder.download(&client, contest_id, output_dir).await?;
        }
        _ => {
            println!("Unsupported service: {}", service_name);
        }
    }
    Ok(())
}

use crate::api::service::{LoginResult, ServiceLogin};
use reqwest::{Client, Url};
use scraper::{Html, Selector};
use std::error::Error;

pub struct AtCoder;

impl ServiceLogin for AtCoder {
    async fn login(
        &self,
        client: &Client,
        username: &str,
        password: &str,
    ) -> Result<LoginResult, Box<dyn Error>> {
        let login_url = Url::parse("https://atcoder.jp/login")?;

        // Step 1: Get CSRF token
        let response = client.get(login_url.clone()).send().await?;
        let body = response.text().await?;
        let document = Html::parse_document(&body);
        let selector = Selector::parse("input[name=\"csrf_token\"]").unwrap();
        let csrf_token = document
            .select(&selector)
            .next()
            .ok_or("CSRF token not found")?
            .value()
            .attr("value")
            .ok_or("CSRF token value missing")?;

        // Step 2: Submit login form
        let params = [
            ("username", username),
            ("password", password),
            ("csrf_token", csrf_token),
        ];

        let response = client.post(login_url).form(&params).send().await?;

        // Step 3: Check if login was successful
        if response.url().path() != "/login" {
            Ok(LoginResult {
                success: true,
                message: Some("Login successful!".to_string()),
            })
        } else {
            Ok(LoginResult {
                success: false,
                message: Some("Login failed.".to_string()),
            })
        }
    }
}

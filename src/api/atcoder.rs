use crate::api::service::{LoginResult, ServiceDownload, ServiceLogin};
use crate::utils::fs_utils;
use regex::Regex;
use reqwest::{Client, Url};
use scraper::{Html, Selector};
use std::collections::HashSet;
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

impl ServiceDownload for AtCoder {
    async fn download(
        &self,
        client: &Client,
        contest_id: &str,
        output_dir: Option<&str>,
    ) -> Result<(), Box<dyn Error>> {
        let output_base_dir = output_dir.unwrap_or(".");

        // Step 1: Fetch tasks
        let tasks = self.fetch_tasks(client, contest_id).await?;

        // Step 2: Create contest directory
        let contest_dir = self.create_contest_directory(output_base_dir, contest_id)?;

        // Step 3: Download and save samples
        self.download_and_save_samples(client, contest_id, &contest_dir, &tasks)
            .await?;

        Ok(())
    }
}

impl AtCoder {
    async fn fetch_tasks(
        &self,
        client: &Client,
        contest_id: &str,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let url = format!("https://atcoder.jp/contests/{}/tasks", contest_id);
        let response = client.get(&url).send().await?;

        if response.status().as_u16() == 404 {
            return Err("Contest not found".into());
        }

        let body = response.text().await?;
        let document = Html::parse_document(&body);
        let selector = Selector::parse("a").unwrap();

        let tasks = document
            .select(&selector)
            .filter_map(|element| element.value().attr("href"))
            .filter(|href| href.starts_with(&format!("/contests/{}/tasks/", contest_id)))
            .map(|href| href.split('/').nth(4).unwrap().to_string())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        Ok(tasks)
    }

    fn create_contest_directory(
        &self,
        base_dir: &str,
        contest_id: &str,
    ) -> Result<String, Box<dyn Error>> {
        let contest_dir = format!("{}/{}", base_dir, contest_id);
        fs_utils::create_directory(&contest_dir)?;
        Ok(contest_dir)
    }

    async fn download_and_save_samples(
        &self,
        client: &Client,
        contest_id: &str,
        contest_dir: &str,
        tasks: &[String],
    ) -> Result<(), Box<dyn Error>> {
        let input_regex = Regex::new(r"入力例|Sample Input")?;
        let output_regex = Regex::new(r"出力例|Sample Output")?;
        let target_h3_regex = Regex::new(r"入力例|出力例|Sample Input|Sample Output")?;

        for task in tasks {
            self.download_task_samples(
                client,
                contest_id,
                task,
                contest_dir,
                &input_regex,
                &output_regex,
                &target_h3_regex,
            )
            .await?;
        }

        Ok(())
    }

    async fn download_task_samples(
        &self,
        client: &Client,
        contest_id: &str,
        task_id: &str,
        contest_dir: &str,
        input_regex: &Regex,
        output_regex: &Regex,
        target_h3_regex: &Regex,
    ) -> Result<(), Box<dyn Error>> {
        let task_url = format!(
            "https://atcoder.jp/contests/{}/tasks/{}",
            contest_id, task_id
        );
        let response = client.get(&task_url).send().await?;
        let body = response.text().await?;
        let document = Html::parse_document(&body);

        let title_selector = Selector::parse("title").unwrap();
        let title = document
            .select(&title_selector)
            .next()
            .ok_or("Title not found")?
            .text()
            .collect::<String>();
        let problem_prefix = title.split(" - ").next().unwrap().to_lowercase();

        let task_test_dir = format!("{}/{}/tests", contest_dir, problem_prefix);
        fs_utils::create_directory(&task_test_dir)?;

        let part_selector = Selector::parse(".part").unwrap();
        let h3_selector = Selector::parse("h3").unwrap();
        let pre_selector = Selector::parse("pre").unwrap();

        for part in document.select(&part_selector) {
            if let Some(h3) = part.select(&h3_selector).next() {
                let header_text = h3.text().collect::<String>().trim().to_string();
                if target_h3_regex.is_match(&header_text) {
                    for pre in part.select(&pre_selector) {
                        let sample = pre.text().collect::<String>();
                        let sample_number = header_text
                            .split_whitespace()
                            .last()
                            .ok_or("Sample number not found")?;

                        if input_regex.is_match(&header_text) {
                            self.save_sample(&task_test_dir, sample_number, "in", &sample)?;
                        } else if output_regex.is_match(&header_text) {
                            self.save_sample(&task_test_dir, sample_number, "out", &sample)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn save_sample(
        &self,
        task_test_dir: &str,
        sample_number: &str,
        ext: &str,
        content: &str,
    ) -> Result<(), Box<dyn Error>> {
        let filename = format!("sample-{}.{}", sample_number, ext);
        let file_path = format!("{}/{}", task_test_dir, filename);
        fs_utils::create_file_with_content(&file_path, content)?;
        Ok(())
    }
}

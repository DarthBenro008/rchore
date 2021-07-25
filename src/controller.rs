use crate::api::{ApiClient, ApiTaskList};
use anyhow;
use reqwest::header;
use std::env;

pub fn test_fetch() -> anyhow::Result<()> {
    let token = env::var("ID").unwrap();
    let formatted_token = format!("{} {}", "Bearer ", token);
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&formatted_token).unwrap(),
    );
    let reqwest_client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;
    let google_api_client = ApiClient {
        base_url: String::from("https://tasks.googleapis.com/tasks/v1"),
        client: reqwest_client,
    };
    let resp = google_api_client.fetch_tasklist();
    println!("{:#?}", resp);
    Ok(())
}

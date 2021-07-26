use crate::service::google_api::GoogleApiClient;
use crate::service::google_tasklist::ApiTaskList;
use crate::service::google_tasks::ApiTasks;
use anyhow;
use reqwest::header;
use std::env;

pub fn test_fetch() -> anyhow::Result<()> {
    let token = env::var("ID").unwrap();
    let formatted_token = format!("{} {}", "Bearer ", token);
    println!("{}", formatted_token);
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&formatted_token).unwrap(),
    );
    let reqwest_client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;
    let mut google_api_client = GoogleApiClient {
        base_url: String::from("https://tasks.googleapis.com/tasks/v1"),
        client: reqwest_client,
        tasklist: None,
    };
    let resp = google_api_client.fetch_tasklist();
    match resp {
        Ok(task_response) => {
            let first_tasklist = task_response.items.get(0);
            match first_tasklist {
                Some(task_list) => {
                    google_api_client.tasklist = Some(String::from(task_list.id.as_ref().unwrap()))
                }
                _ => println!("Some error occured in fetching tasklists"),
            }
        }
        _ => println!("{:#?}", resp),
    }
    let new_resp = google_api_client
        .fetch_task("MDI5MDM4MTYwNzQzNjY1MTk0NTc6MDoyNzY5NTcwMTEzOTYwNzE4".to_string());
    println!("{:#?}", new_resp);
    Ok(())
}

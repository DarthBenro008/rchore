use crate::service::google_tasklist::ApiTaskList;
use reqwest::header;
use std::env;

pub struct GoogleApiClient {
    pub client: reqwest::blocking::Client,
    pub base_url: String,
    pub tasklist: Option<String>,
}

impl GoogleApiClient {
    pub fn new() -> GoogleApiClient {
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
            .build();
        let mut google_api_client = GoogleApiClient {
            base_url: String::from("https://tasks.googleapis.com/tasks/v1"),
            client: reqwest_client.unwrap(),
            tasklist: None,
        };
        let resp = google_api_client.fetch_tasklist();
        match resp {
            Ok(task_response) => {
                let first_tasklist = task_response.items.get(0);
                match first_tasklist {
                    Some(task_list) => {
                        google_api_client.tasklist =
                            Some(String::from(task_list.id.as_ref().unwrap()))
                    }
                    _ => println!("Some error occured in fetching tasklists"),
                }
            }
            _ => println!("{:#?}", resp),
        }
        return google_api_client;
    }
}

pub fn format_base_url(base_url: &String, route: String) -> String {
    return format!("{}{}", base_url, route);
}

pub fn format_task_url(base_url: &String, route: String, task_id: String) -> String {
    return format!("{}/{}", format_base_url(&base_url, route), task_id);
}

pub fn format_specific_task_url(
    base_url: &String,
    route: String,
    task_id: String,
    task_route: String,
) -> String {
    return format!(
        "{}/{}",
        format_task_url(base_url, route, task_id),
        task_route
    );
}

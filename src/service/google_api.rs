use crate::oauth;
use crate::oauth::oauth_login;
use crate::printer::print_red;
use crate::service::database_api::TasksDatabase;
use crate::service::google_tasklist::ApiTaskList;
use reqwest::header;

pub struct GoogleApiClient {
    pub client: reqwest::blocking::Client,
    pub base_url: String,
    pub tasklist: Option<String>,
    pub localdb: TasksDatabase,
}

impl GoogleApiClient {
    pub fn new(tasks_database: TasksDatabase) -> GoogleApiClient {
        if let Err(_err) = &tasks_database.get_token() {
            let res = oauth_login(&tasks_database);
            if let Err(_err) = res {
                print_red("logging you in, please try again");
                std::process::exit(1);
            };
        };
        if tasks_database.is_token_refresh_required().unwrap() {
            let res = oauth::get_new_access_token(&tasks_database);
            if let Err(_err) = res {
                print_red("logging you in, please try again");
                std::process::exit(1);
            };
        }
        let token = &tasks_database.get_token().unwrap();
        let formatted_token = format!("{} {}", "Bearer ", token);
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&formatted_token).unwrap(),
        );
        let reqwest_client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build();
        if let Err(_err) = &tasks_database.get_default_tasklist() {
            let mut google_api_client = GoogleApiClient {
                base_url: String::from("https://tasks.googleapis.com/tasks/v1"),
                client: reqwest_client.unwrap(),
                tasklist: None,
                localdb: tasks_database,
            };
            let _ = google_api_client.fetch_tasklist(true);
            return google_api_client;
        };
        let saved_default_tasklist_title = tasks_database.get_default_tasklist().unwrap();
        GoogleApiClient {
            base_url: String::from("https://tasks.googleapis.com/tasks/v1"),
            client: reqwest_client.unwrap(),
            tasklist: Some(saved_default_tasklist_title.0),
            localdb: tasks_database,
        }
    }
}

pub fn format_base_url(base_url: &str, route: String) -> String {
    format!("{}{}", base_url, route)
}

pub fn format_task_url(base_url: &str, route: String, task_id: String) -> String {
    format!("{}/{}", format_base_url(base_url, route), task_id)
}

pub fn format_specific_task_url(
    base_url: &str,
    route: String,
    task_id: String,
    task_route: String,
) -> String {
    format!(
        "{}/{}",
        format_task_url(base_url, route, task_id),
        task_route
    )
}

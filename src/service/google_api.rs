pub struct GoogleApiClient {
    pub client: reqwest::blocking::Client,
    pub base_url: String,
    pub tasklist: Option<String>,
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

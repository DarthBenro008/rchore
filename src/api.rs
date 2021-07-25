use crate::models::tasklist::TaskListResponse;

pub struct ApiClient {
    pub client: reqwest::blocking::Client,
    pub base_url: String,
}

pub trait ApiTaskList {
    fn fetch_tasklist(&self) -> Result<TaskListResponse, Box<dyn std::error::Error>>;
    fn create_tasklist(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn delete_tasklist(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn update_tasklist(&self) -> Result<(), Box<dyn std::error::Error>>;
}

trait ApiTasks {
    fn fetchAllTasks(&self);
    fn fetchTask(&self);
    fn deleteTask(&self);
    fn updateTask(&self);
    fn clearCompletedTasks(&self);
}

fn format_base_url(base_url: &String, route: String) -> String {
    let url = format!("{}{}", base_url, route);
    return url;
}

impl ApiTaskList for ApiClient {
    fn fetch_tasklist(&self) -> Result<TaskListResponse, Box<dyn std::error::Error>> {
        let url = format_base_url(&self.base_url, String::from("/users/@me/lists"));
        let resp = self.client.get(url).send()?.json::<TaskListResponse>()?;
        Ok(resp)
    }

    fn create_tasklist(&self) -> Result<(), Box<dyn std::error::Error>> {
        let resp = self
            .client
            .get(format_base_url(
                &self.base_url,
                String::from("/users/@me/lists"),
            ))
            .send()?
            .json::<TaskListResponse>()?;
        Ok(())
    }

    fn delete_tasklist(&self) -> Result<(), Box<dyn std::error::Error>> {
        let resp = self
            .client
            .get(format_base_url(
                &self.base_url,
                String::from("/users/@me/lists"),
            ))
            .send()?
            .json::<TaskListResponse>()?;
        Ok(())
    }

    fn update_tasklist(&self) -> Result<(), Box<dyn std::error::Error>> {
        let resp = self
            .client
            .get(format_base_url(
                &self.base_url,
                String::from("/users/@me/lists"),
            ))
            .send()?
            .json::<TaskListResponse>()?;
        Ok(())
    }
}

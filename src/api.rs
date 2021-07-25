use crate::models::tasklist::{TaskList, TaskListResponse};

pub struct ApiClient {
    pub client: reqwest::blocking::Client,
    pub base_url: String,
}

pub trait ApiTaskList {
    fn fetch_tasklist(&self) -> Result<TaskListResponse, Box<dyn std::error::Error>>;
    fn create_tasklist(&self, title: String) -> Result<(), Box<dyn std::error::Error>>;
    fn delete_tasklist(&self, id: String) -> Result<(), Box<dyn std::error::Error>>;
    fn update_tasklist(&self, id: String, title: String) -> Result<(), Box<dyn std::error::Error>>;
}

trait ApiTasks {
    fn fetch_all_tasks(&self);
    fn fetch_task(&self);
    fn delete_task(&self);
    fn update_task(&self);
    fn clear_completed_tasks(&self);
}

fn format_base_url(base_url: &String, route: String) -> String {
    return format!("{}{}", base_url, route);
}

fn format_task_url(base_url: &String, route: String, task_id: String) -> String {
    return format!("{}/{}", format_base_url(&base_url, route), task_id);
}

impl ApiTaskList for ApiClient {
    fn fetch_tasklist(&self) -> Result<TaskListResponse, Box<dyn std::error::Error>> {
        let url = format_base_url(&self.base_url, String::from("/users/@me/lists"));
        let resp = self.client.get(url).send()?.json::<TaskListResponse>()?;
        Ok(resp)
    }

    fn create_tasklist(&self, title: String) -> Result<(), Box<dyn std::error::Error>> {
        let new_task_list = TaskList::new(title);
        let resp = self
            .client
            .post(format_base_url(
                &self.base_url,
                String::from("/users/@me/lists"),
            ))
            .json(&new_task_list)
            .send()?;
        println!("{:#?}", resp);
        Ok(())
    }

    fn delete_tasklist(&self, id: String) -> Result<(), Box<dyn std::error::Error>> {
        let del_url = format_task_url(&self.base_url, "/users/@me/lists".to_string(), id);
        let resp = self.client.delete(del_url).send()?;
        println!("{:#?}", resp);
        Ok(())
    }

    fn update_tasklist(&self, id: String, title: String) -> Result<(), Box<dyn std::error::Error>> {
        let patch_url = format_task_url(&self.base_url, "/users/@me/lists".to_string(), id);
        let task_list = TaskList::new(title);
        let resp = self.client.patch(patch_url).json(&task_list).send()?;
        println!("{:#?}", resp);
        Ok(())
    }
}

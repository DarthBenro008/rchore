use super::google_api::{format_base_url, format_task_url, GoogleApiClient};
use crate::models::tasklist::{TaskList, TaskListResponse};

pub trait ApiTaskList {
    fn fetch_tasklist(&self) -> Result<TaskListResponse, Box<dyn std::error::Error>>;
    fn create_tasklist(&self, title: String) -> Result<TaskList, Box<dyn std::error::Error>>;
    fn delete_tasklist(&self, id: String) -> Result<(), Box<dyn std::error::Error>>;
    fn update_tasklist(
        &self,
        id: String,
        title: String,
    ) -> Result<TaskList, Box<dyn std::error::Error>>;
}

impl ApiTaskList for GoogleApiClient {
    fn fetch_tasklist(&self) -> Result<TaskListResponse, Box<dyn std::error::Error>> {
        let url = format_base_url(&self.base_url, String::from("/users/@me/lists"));
        let resp = self.client.get(url).send()?.json::<TaskListResponse>()?;
        Ok(resp)
    }

    fn create_tasklist(&self, title: String) -> Result<TaskList, Box<dyn std::error::Error>> {
        let new_task_list = TaskList::new(title);
        let resp = self
            .client
            .post(format_base_url(
                &self.base_url,
                String::from("/users/@me/lists"),
            ))
            .json(&new_task_list)
            .send()?
            .json::<TaskList>()?;
        Ok(resp)
    }

    fn delete_tasklist(&self, id: String) -> Result<(), Box<dyn std::error::Error>> {
        let del_url = format_task_url(&self.base_url, "/users/@me/lists".to_string(), id);
        let resp = self.client.delete(del_url).send()?;
        if resp.status() != 204 {
            Err("Unable to delete list!")?
        }
        Ok(())
    }

    fn update_tasklist(
        &self,
        id: String,
        title: String,
    ) -> Result<TaskList, Box<dyn std::error::Error>> {
        let patch_url = format_task_url(&self.base_url, "/users/@me/lists".to_string(), id);
        let task_list = TaskList::new(title);
        let resp = self
            .client
            .patch(patch_url)
            .json(&task_list)
            .send()?
            .json::<TaskList>()?;
        Ok(resp)
    }
}

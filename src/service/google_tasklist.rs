use super::google_api::{format_base_url, format_task_url, GoogleApiClient};
use crate::models::tasklist::{TaskList, TaskListResponse};
use crate::oauth::get_new_access_token;
use crate::printer::print_red;

pub trait ApiTaskList {
    fn fetch_tasklist(
        &mut self,
        default: bool,
    ) -> Result<TaskListResponse, Box<dyn std::error::Error>>;
    fn create_tasklist(&self, title: String) -> Result<TaskList, Box<dyn std::error::Error>>;
    fn delete_tasklist(&self, id: String) -> Result<(), Box<dyn std::error::Error>>;
    fn update_tasklist(
        &self,
        id: String,
        title: String,
    ) -> Result<TaskList, Box<dyn std::error::Error>>;
}

impl ApiTaskList for GoogleApiClient {
    fn fetch_tasklist(
        &mut self,
        default: bool,
    ) -> Result<TaskListResponse, Box<dyn std::error::Error>> {
        let url = format_base_url(&self.base_url, String::from("/users/@me/lists"));
        let resp = self.client.get(url).send()?;
        if resp.status() != 200 {
            get_new_access_token(&self.localdb)?;
            let token = self.localdb.get_token()?;
            let new_client = GoogleApiClient::new_token_client(token);
            self.client = new_client;
            self.fetch_tasklist(false)?;
        }
        let task_list = resp.json::<TaskListResponse>()?;
        if default {
            let first_tasklist = task_list.items.get(0);
            match first_tasklist {
                Some(task_list) => {
                    &self.localdb.insert_default_tasklist(
                        task_list.id.as_ref().unwrap().to_string(),
                        String::from(&task_list.title),
                    );
                    self.tasklist = Some(String::from(task_list.id.as_ref().unwrap()))
                }
                _ => print_red("fetching tasklist"),
            }
        }
        Ok(task_list)
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
            return Err("Unable to delete list!".into());
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

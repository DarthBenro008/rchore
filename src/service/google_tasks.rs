use super::google_api::{format_specific_task_url, GoogleApiClient};
use crate::models::tasks::{TaskResponse, Tasks};
use crate::oauth::get_new_access_token;

pub trait ApiTasks {
    fn fetch_all_tasks(
        &self,
        show_hidden: bool,
    ) -> Result<TaskResponse, Box<dyn std::error::Error>>;
    fn fetch_task(&self, id: String) -> Result<Tasks, Box<dyn std::error::Error>>;
    fn delete_task(&self, id: String) -> Result<(), Box<dyn std::error::Error>>;
    fn update_task(&self, updated_task: Tasks) -> Result<Tasks, Box<dyn std::error::Error>>;
    fn clear_completed_tasks(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn add_task(&self, task: Tasks) -> Result<Tasks, Box<dyn std::error::Error>>;
}

impl ApiTasks for GoogleApiClient {
    fn add_task(&self, task: Tasks) -> Result<Tasks, Box<dyn std::error::Error>> {
        let url = format_specific_task_url(
            &self.base_url,
            String::from("/lists"),
            self.tasklist.as_ref().unwrap().to_string(),
            String::from("tasks"),
        );
        let resp = self.client.post(url).json(&task).send()?;
        if resp.status() != 200 {
            get_new_access_token(&self.localdb)?;
            self.add_task(task)?;
        }
        let tasks = resp.json::<Tasks>()?;
        Ok(tasks)
    }
    fn fetch_all_tasks(
        &self,
        show_hidden: bool,
    ) -> Result<TaskResponse, Box<dyn std::error::Error>> {
        let query_params = if show_hidden {
            String::from("tasks?showCompleted=true&showHidden=true")
        } else {
            String::from("tasks?showCompleted=true")
        };
        let url = format_specific_task_url(
            &self.base_url,
            String::from("/lists"),
            self.tasklist.as_ref().unwrap().to_string(),
            query_params,
        );
        let resp = self.client.get(url).send()?;
        if resp.status() != 200 {
            get_new_access_token(&self.localdb)?;
            self.fetch_all_tasks(show_hidden)?;
        }
        let tasks_response = resp.json::<TaskResponse>()?;
        Ok(tasks_response)
    }

    fn fetch_task(&self, id: String) -> Result<Tasks, Box<dyn std::error::Error>> {
        let url = format_specific_task_url(
            &self.base_url,
            String::from("/lists"),
            self.tasklist.as_ref().unwrap().to_string(),
            format!("{}/{}", "tasks", id),
        );
        let resp = self.client.get(url).send()?.json::<Tasks>()?;
        Ok(resp)
    }

    fn delete_task(&self, id: String) -> Result<(), Box<dyn std::error::Error>> {
        let url = format_specific_task_url(
            &self.base_url,
            String::from("/lists"),
            self.tasklist.as_ref().unwrap().to_string(),
            format!("{}/{}", "tasks", id),
        );
        let resp = self.client.delete(url).send()?;
        if resp.status() != 204 {
            return Err("Failed to call delete".into());
        }
        Ok(())
    }

    fn update_task(&self, updated_task: Tasks) -> Result<Tasks, Box<dyn std::error::Error>> {
        let id = &updated_task.id;
        let url = format_specific_task_url(
            &self.base_url,
            String::from("/lists"),
            self.tasklist.as_ref().unwrap().to_string(),
            format!("{}/{}", "tasks", id.as_ref().unwrap()),
        );
        let resp = self
            .client
            .patch(url)
            .json(&updated_task)
            .send()?
            .json::<Tasks>()?;
        Ok(resp)
    }

    fn clear_completed_tasks(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = format_specific_task_url(
            &self.base_url,
            String::from("/lists"),
            self.tasklist.as_ref().unwrap().to_string(),
            String::from("clear"),
        );
        let resp = self.client.post(url).body("").send()?;
        if resp.status() != 204 {
            return Err("Failed to clear".into());
        }
        Ok(())
    }
}

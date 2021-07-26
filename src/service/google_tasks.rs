use super::google_api::{format_specific_task_url, GoogleApiClient};
use crate::models::tasks::{TaskResponse, Tasks};

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
        let resp = self.client.post(url).json(&task).send()?.json::<Tasks>()?;
        Ok(resp)
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
        let resp = self.client.get(url).send()?.json::<TaskResponse>()?;
        Ok(resp)
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
        println!("{:#?}", resp);
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
        println!("{:#?}", resp);
        Ok(())
    }
}

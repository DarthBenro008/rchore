pub trait ApiTasks {
    fn fetch_all_tasks(&self);
    fn fetch_task(&self);
    fn delete_task(&self);
    fn update_task(&self);
    fn clear_completed_tasks(&self);
}

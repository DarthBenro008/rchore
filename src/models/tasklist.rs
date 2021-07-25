#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskListResponse {
    pub kind: String,
    pub etag: String,
    pub items: Vec<TaskList>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskList {
    pub kind: String,
    pub id: String,
    pub etag: String,
    pub title: String,
    pub updated: String,
    pub self_link: String,
}

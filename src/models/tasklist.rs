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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub etag: String,
    pub title: String,
    pub updated: String,
    pub self_link: String,
}

impl TaskList {
    pub fn new(title: String) -> TaskList {
        TaskList {
            kind: "".to_string(),
            id: None,
            etag: "".to_string(),
            updated: "".to_string(),
            self_link: "".to_string(),
            title: title,
        }
    }
}

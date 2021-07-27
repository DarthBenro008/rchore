use std::fmt;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskResponse {
    pub kind: String,
    pub etag: String,
    pub items: Vec<Tasks>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tasks {
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none", skip_deserializing)]
    pub due: Option<String>,
}

impl Tasks {
    pub fn new(id: Option<String>, title: String, notes: String, status: String) -> Tasks {
        Tasks {
            kind: "".to_string(),
            id: id,
            etag: None,
            title: title,
            updated: None,
            self_link: None,
            position: None,
            notes: Some(notes),
            status: status,
            due: None,
        }
    }
}

impl fmt::Display for Tasks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:<50} [{}]", self.title, self.status)
    }
}

use console::style;
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub due: Option<String>,
}

impl Tasks {
    pub fn new(id: Option<String>, title: String, notes: String, status: String) -> Tasks {
        Tasks {
            kind: "".to_string(),
            id,
            etag: None,
            title,
            updated: None,
            self_link: None,
            position: None,
            notes: Some(notes),
            status,
            due: None,
        }
    }

    pub fn get_sanitised_data(&self) -> (String, String, String, String) {
        let status = if self.status == "needsAction" {
            String::from("Incomplete")
        } else {
            String::from("Completed")
        };
        let due = if self.due == None {
            String::from("Not specified")
        } else {
            String::from(self.due.as_ref().unwrap())
        };
        let notes = if self.notes == None {
            String::from("No note was added")
        } else {
            String::from(self.notes.as_ref().unwrap())
        };
        (String::from(&self.title), status, notes, due)
    }
}

impl fmt::Display for Tasks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.status == "needsAction" {
            String::from("Incomplete")
        } else {
            String::from("Completed")
        };
        let due = if self.due == None {
            String::from("Not specified")
        } else {
            String::from(self.due.as_ref().unwrap())
        };
        let notes = if self.notes == None {
            String::from("No note added")
        } else {
            String::from(self.notes.as_ref().unwrap())
        };
        write!(
            f,
            "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
            style(&self.title).for_stdout().green(),
            notes,
            status,
            due
        )
    }
}

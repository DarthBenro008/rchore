use chrono::DateTime;
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
    #[serde(default)]
    pub notes: String,
    pub status: String,
    #[serde(default)]
    pub due: String,
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
            notes,
            status,
            due: String::from(""),
        }
    }

    pub fn get_sanitised_data(&self) -> (String, String, String, String) {
        let status = if self.status == "needsAction" {
            String::from("Incomplete")
        } else {
            String::from("Completed")
        };
        let due = if self.due.is_empty() {
            String::from("Not specified")
        } else {
            let datetime = DateTime::parse_from_rfc3339(&self.due).unwrap();
            let newdate = datetime.format("%d/%m/%Y");
            format!("{}", newdate)
        };
        let notes = if self.notes.is_empty() {
            String::from("No note was added")
        } else {
            String::from(&self.notes)
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
        let due = if self.due.is_empty() {
            String::from("Not specified")
        } else {
            String::from(&self.due)
        };
        let notes = if self.notes.is_empty() {
            String::from("No note added")
        } else {
            String::from(&self.notes)
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

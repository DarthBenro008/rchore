use chrono::DateTime;
use console::style;
use std::fmt;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskResponse {
    pub kind: String,
    pub etag: String,
    #[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
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
            parent: None,
        }
    }

    pub fn clone(&self) -> Tasks {
        Tasks {
            kind: String::from(&self.kind),
            id: Some(String::from(self.id.as_ref().unwrap())),
            etag: Some(String::from(self.id.as_ref().unwrap())),
            title: String::from(&self.title),
            updated: Some(String::from(self.updated.as_ref().unwrap())),
            self_link: Some(String::from(self.self_link.as_ref().unwrap())),
            position: Some(String::from(self.position.as_ref().unwrap())),
            notes: String::from(&self.notes),
            status: String::from(&self.status),
            due: String::from(&self.due),
            parent: self.parent.clone(),
        }
    }

    pub fn get_sanitised_data(&self, tasks: &[Tasks]) -> (String, String, String, String, String) {
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
        let parent = self.parent.clone().unwrap_or("No parent".into());

        println!("{self:?}");

        let parent = tasks
            .iter()
            .find(|t| t.id == Some(parent.clone()))
            .map(|t| t.title.clone())
            .unwrap_or("Parent not found!".to_string());
        (
            String::from(&self.title),
            status,
            notes,
            due,
            parent.to_string(),
        )
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
        let parent = self.parent.clone().unwrap_or("No parent".into());
        write!(
            f,
            "{0: <10} | {1: <10} | {2: <10} | {3: <10} | {4: <10}",
            style(&self.title).for_stdout().green(),
            notes,
            status,
            due,
            parent,
        )
    }
}

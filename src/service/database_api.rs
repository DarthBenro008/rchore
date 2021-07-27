use crate::models::tasks::Tasks;
use anyhow::anyhow;

pub struct TasksDatabase {
    db: sled::Db,
}

#[derive(Default, serde::Serialize, serde::Deserialize, PartialEq, Debug)]
struct TasksList(Vec<Tasks>);

impl TasksDatabase {
    pub fn new() -> TasksDatabase {
        match home::home_dir() {
            Some(mut path) => {
                path.push(".r_chore");
                let db: sled::Db = sled::open(path).unwrap();
                return TasksDatabase { db: db };
            }
            None => {
                let db: sled::Db = sled::open("rchore_db").unwrap();
                return TasksDatabase { db: db };
            }
        }
    }

    pub fn insert_tasks(&self, taskslist: Vec<Tasks>) -> anyhow::Result<()> {
        let task_struct: TasksList = TasksList(taskslist);
        let bytes = bincode::serialize(&task_struct)?;
        self.db.insert("data", bytes)?;
        Ok(())
    }

    pub fn get_data(&self) -> anyhow::Result<Vec<Tasks>> {
        match self.db.get("data")? {
            Some(bytes) => {
                let tasks_list: TasksList = bincode::deserialize(&bytes).unwrap();
                Ok(tasks_list.0)
            }
            None => Err(anyhow!("Error!")),
        }
    }

    pub fn insert_token(&self, token: String) -> anyhow::Result<()> {
        let bytes = bincode::serialize(&token)?;
        self.db.insert("token", bytes)?;
        Ok(())
    }

    pub fn get_token(&self) -> anyhow::Result<String> {
        match self.db.get("token")? {
            Some(bytes) => {
                let token: String = bincode::deserialize(&bytes)?;
                Ok(token)
            }
            None => Err(anyhow!("Error!")),
        }
    }
}

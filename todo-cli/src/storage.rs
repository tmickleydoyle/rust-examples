use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

use crate::task::Task;

const DATA_FILE: &str = "tasks.json";

pub struct Storage {
    file_path: String,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            file_path: DATA_FILE.to_string(),
        }
    }

    /// Creates a Storage instance with a custom file path.
    /// 
    /// This function is provided for future extensibility,
    /// allowing users to specify a custom location for the tasks file.
    #[allow(dead_code)]
    pub fn with_path(file_path: String) -> Self {
        Storage { file_path }
    }

    pub fn save_tasks(&self, tasks: &[Task]) -> io::Result<()> {
        let json = serde_json::to_string_pretty(tasks)?;
        let mut file = File::create(&self.file_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn load_tasks(&self) -> io::Result<Vec<Task>> {
        if !Path::new(&self.file_path).exists() {
            return Ok(Vec::new());
        }

        let mut file = File::open(&self.file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        if contents.trim().is_empty() {
            return Ok(Vec::new());
        }

        match serde_json::from_str(&contents) {
            Ok(tasks) => Ok(tasks),
            Err(e) => {
                eprintln!("Error parsing tasks file: {}", e);
                Ok(Vec::new())
            }
        }
    }

    pub fn add_task(&self, task: Task) -> io::Result<()> {
        let mut tasks = self.load_tasks()?;
        tasks.push(task);
        self.save_tasks(&tasks)
    }

    pub fn complete_task(&self, task_id: &str) -> io::Result<bool> {
        let mut tasks = self.load_tasks()?;
        let mut found = false;

        for task in &mut tasks {
            if task.id == task_id {
                task.mark_completed();
                found = true;
                break;
            }
        }

        if found {
            self.save_tasks(&tasks)?;
        }

        Ok(found)
    }
}
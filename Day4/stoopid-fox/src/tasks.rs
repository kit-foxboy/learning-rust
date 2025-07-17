pub mod tasks {
    use std::fs;
    use serde::{Serialize, Deserialize};
    use crate::cli::cli;

    pub struct TaskManager {
        tasks: Vec<Task>,
        greatest_id: usize
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Task {
        id: usize,
        description: String,
        is_priority: bool,
        is_completed: bool
    }

    impl TaskManager {

        const FILEPATH: &str = "data/tasks.json";

        pub fn new() -> Self {
            TaskManager { tasks: Self::load_tasks().tasks , greatest_id: 0 }
        }

        pub fn load_tasks() -> TaskManager {
            match fs::read_to_string(Self::FILEPATH) {
                Ok(data) => {
                    // deserialize the JJSON tasks and set the highest ID
                    let tasks: Vec<Task> = serde_json::from_str(&data).unwrap_or_default();
                    let greatest_id = tasks.iter().map(|task| task.id).max().unwrap_or(0);
                    TaskManager { tasks, greatest_id }
                },
                Err(_) => TaskManager { tasks: Vec::new(), greatest_id: 0 }
            }
        }

        pub fn save_tasks(&self) {
            let data = serde_json::to_string_pretty(&self.tasks).unwrap();
            
            fs::write(Self::FILEPATH, data).expect("Something went wrong and I forgotted your tasks ;w;");
        }

        pub fn add_task(&mut self) {
            self.greatest_id += 1; // Increment the greatest ID for new tasks
            let desc = cli::get_input("^w^ Gud idea fren! Enter task description: ");
            let id = self.greatest_id;
            let task = Task {
                id,
                description: desc,
                is_priority: false,
                is_completed: false
            };

            self.tasks.push(task);
            cli::success_message("*Noms on your task, adding it to the list* UwU");
        }

        pub fn add_important_task(&mut self) {
            self.greatest_id += 1; // Increment the greatest ID for new tasks
            let desc = cli::get_input("OwO! Enter important task description: ");
            let id = self.greatest_id;
            let task = Task {
                id,
                description: desc,
                is_priority: true,
                is_completed: false
            };

            self.tasks.push(task);
            cli::success_message("*Noms on your important task with dainty bites, adding it to the list* UwU");
        }

        pub fn view_tasks(&self) {
            if self.tasks.is_empty() {
                cli::error_message("*Whines* No tasks found, you need to add some tasks first! ;w;");
                return;
            }

            cli::success_message("*Wags tail excitedly* Here are your tasks, do I get a treat?:");
            for task in &self.tasks {
                let status = if task.is_completed { "Completed" } else { "Pending" };
                let priority = if task.is_priority { "Important" } else { "Normal" };
                println!("{}: [{}] {} - {}", task.id, priority, task.description, status);
            }
        }

        pub fn mark_task_completed(&mut self) {
            let id = cli::get_input_as_int("Yay! Which ID did you finish like a good boy/girl/edible friend?: ") as usize;
            if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
                task.is_completed = true;
                cli::success_message("*Wags tail happily* Task marked as completed! I yam so proud of u! ^w^");
            } else {
                cli::error_message("*Rolls around on the floor sobbing* No tasks with that id! ;w;");
            }
        }

        pub fn delete_task(&mut self) {
            let id = cli::get_input_as_int("Which task ID do you want to me to hit with a broom? ;w;: ") as usize;
            if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
                self.tasks.remove(pos);
                self.greatest_id -= 1; // Decrement the greatest ID if a task is deleted
                cli::success_message("*Beats the task to death with the broom* Task deleted! Sometimes violence IS the answer! UwU");
            } else {
                cli::error_message("*Whines* No tasks with that id! ;w;");
            }
        }
    }
}
// declare struct Task
struct Task {
    id: usize,
    description: String,
    completed: bool
}

// declare a struct for Todolist (vector) so can be accessible globally
struct TodoList {
    tasks: Vec<Task>
}

// implement method for the list of struct
impl TodoList {
    //adding function for adding task
    fn add_task(&mut self, description: &str)-> Task {
        let id = self.tasks.len() + 1; // make id based on length of list
        let new_task = Task {
            id,
            description: String::from(description),
            completed: false,
        };
        self.tasks.push(new_task.clone()); // Clone the task and add to the vector
        new_task
    }
    
    // adding function for completing task
    fn complete_task(&mut self, id: usize) -> Option<&Task> {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                return Some(task);
            }
        }
        None
    }
    
    // adding function for listing all tasks
    fn list_tasks(&self) {
        println!("ToDo List:");
        for task in &self.tasks {
            println!(
                "ID: {}, Description: {}, Completed: {}",
                task.id, task.description, task.completed
            );
        }
    }
}

fn main() {
    let mut todo_list = TodoList { tasks: Vec::new() };
    let task1 = todo_list.add_task("Study for Bootcamp");
    let task2 = todo_list.add_task("Done Week 1 Homework");

    //testing
    todo_list.list_tasks();
    todo_list.complete_task(task2.id);
    todo_list.list_tasks();
}

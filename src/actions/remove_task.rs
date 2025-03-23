use super::TaskVecBase;
use std::io::{self};


impl TaskVecBase {

    pub fn remove_task(&mut self) {

        if self.task_vec.is_empty() { 
            println!("Nothing to delete. Your task manager is empty..");
            return;
        }
        
        for (index,element) in self.task_vec.iter().enumerate() {
                println!("{} -> {} ", index + 1 , element.description);
            }  
            
        let mut input = String::new();

        println!("Enter the number of the task to delete..");
        io::stdin().read_line(&mut input).expect("Failed to read input..");

        let task_nb: usize = match input.trim().parse::<usize>() {
            Ok(i) if i > 0 && i <= self.task_vec.len() => i - 1,
            _ => {
                println!("Invalid index.");
                return;
            }
        };

        if let Some(task) = self.task_vec.get(task_nb) {
            println!("Task to delete: {}", task.description);
            println!("Please confirm with yes..");
        
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read input..");
            
            match input.trim().to_lowercase().as_str() {

                "yes" => { self.task_vec.remove(task_nb); },

                _     => {println!("Abborted..");},

            }
            
            println!("Task removed successfully.");
        }else {
            println!("Index out of bounds.");
        } 
    }                   
        
}

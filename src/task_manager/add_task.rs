use std::io::{self};
use chrono::Local;

use crate::task_manager::{DoneStatus, Tasker};


use super::TaskBase;

impl TaskBase {

    pub fn add_task(&mut self) {

        println!("Add a new task..");
        

        let mut new_task = Tasker::new(
            String::new(), 
            Local::now(), 
            String::new(), 
            DoneStatus::No );

        io::stdin().read_line(&mut new_task.description).expect("Reading error..");
        new_task.description = new_task.description.trim().to_string();

            
        
        let mut input = String::new();
        println!("Do you want to add a comment ?");

        println!("Please, enter \"yes\" or \"no\"..");

        io::stdin().read_line(&mut input).expect("Failed to read input..");

        match input.trim().to_lowercase().as_str() {

            "yes" =>    { println!("Add your comment and press \"enter\".");
                          input.clear();
                          io::stdin().read_line(&mut input).expect("Failed to read input..");
                          
                          new_task.comment = input.trim().to_string();
                          

                          println!("Comment added."); 
            },

            "no"  =>    { println!("No comment added.");
                          

            },

            _     =>    {println!("Failed to read input..")},
                    
        }
        
        self.task_vec.push(new_task.clone());    
        println!("Task added!");
    }
}
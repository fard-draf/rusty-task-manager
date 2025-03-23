use super::TaskVecBase;
use std::io;



impl TaskVecBase {
    pub fn modify_task(&mut self) {
            
        for (index,element) in self.task_vec.iter().enumerate() {

            println!("{} -> {} // {} // added on {} // Done:{:?} ", 
            index + 1 , 
            element.description, 
            element.comment, 
            element.creation_date.format("%d/%m/%y %H:%M"),
            element.done,
            );
        }

        let mut input = String::new();
            
            println!("Enter the number of the task to modify..");
            io::stdin().read_line(&mut input).expect("Failed to read input..");
        
        let task_nb: usize = match input.trim().parse::<usize>() {
            
            Ok(i) if i > 0 && i <= self.task_vec.len() => i - 1,
            _ => {
                println!("Invalid index.");
                return;
            }
        };

        
        
        if let Some(task) = self.task_vec.get_mut(task_nb) {
            
            println!("Task to modify: {}", task.description);
            println!("Please enter the new value:");
        
            input.clear();
            
            io::stdin().read_line(&mut input).expect("Failed to read input..");
            task.description = input.trim().to_string();                
            
            println!("Do you want to add or modify a comment ?");
            println!("Please, enter \"yes\" or \"no\"..");
            
            input.clear();
            
            io::stdin().read_line(&mut input).expect("Failed to read input..");
            
            match input.trim().to_lowercase().as_str() {
                
                "yes" =>    { println!("Add your comment and press \"enter\".");
                            input.clear();
                            io::stdin().read_line(&mut input).expect("Failed to read input..");
                            task.comment = input.trim().to_string();
                            println!("Comment added."); 
                },
                "no"  =>    { println!("No comment added.");
                              
                },
                _     =>    {println!("Failed to read input..")},
                            
            }
            

            
            println!("Is the task is complete ? ");
            println!("Please enter \"yes\" or \"no\"..");

            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read input..");

            task.change_status(input.as_str());   



            println!("Task updated successfully.");
        } else {
            println!("Index out of bounds.");
        }     
      
    }
}

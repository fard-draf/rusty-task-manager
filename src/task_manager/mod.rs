use std::io::{self, Write};
use::chrono::DateTime;
use chrono::Local;

mod add_task;
mod remove_task;
mod task_list;
mod modify_task;

#[derive(Debug, Clone)]
pub struct Tasker {
    description: String,
    creation_date: DateTime<Local>,
    comment: String,
    done: DoneStatus,
}
#[derive(Debug, Clone)]
pub enum DoneStatus {
    Yes,
    No,    
}

pub struct TaskBase {
    pub task_vec: Vec<Tasker>,
}

impl Tasker {

    fn new(description: String, creation_date: DateTime<Local>, comment: String, done: DoneStatus) -> Self {
        Self {
            description,
            creation_date,
            comment,
            done,
        }
    }
}


impl TaskBase {
    pub fn new() -> Self {
        Self {task_vec: Vec::new()}
    }


    fn display_menu(&self) {
        println!("----Task Manager----");
        println!("1 => Add a task");
        println!("2 => Task list");
        println!("3 => Modify Task");
        println!("4 => Remove Task");
        println!("5 => Quit");
        print!("Choose an option..");
        io::stdout().flush().unwrap();
    }

    pub fn run(&mut self) {

        loop {
            self.display_menu();
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Reading error..");
           
            match choice.trim() {
                "1" => self.add_task(),
                "2" => self.task_list(),
                "3" => self.modify_task(),
                "4" => self.remove_task(),
                "5" =>  println!("{:?}", self.task_vec),

                _ => break,
                
            }
        }
    }



    

    
}

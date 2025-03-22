use super::TaskBase;



impl TaskBase {

    pub fn task_list(&self) {
        println!("----Task List----");
        for (index, task) in self.task_vec.iter().enumerate() {
            println!("{}. {} // {} // Terminated: {:?}", index +1, task.description, task.comment, task.done)
        }

    }
}
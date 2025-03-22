mod task_manager;

use task_manager::TaskBase;


pub fn main() {
    let mut manager = TaskBase::new();
    manager.run();
}
use tasks_manager::actions;
use actions::TaskVecBase;


pub fn main() {
    let mut manager = TaskVecBase::new();
    manager.run();
}
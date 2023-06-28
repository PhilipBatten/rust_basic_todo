use super::Command;

pub struct RemoveCommand {
    args: Vec<String>,
}

impl RemoveCommand {
    pub fn new(args: Vec<String>) -> Self {
        RemoveCommand {
            args
        }
    }
}

impl Command for RemoveCommand {
    fn handle(&self) -> i32 {
        crate::database::remove_todo_by_id(self.args[2].parse::<i32>().unwrap()).unwrap();
        0
    }
}

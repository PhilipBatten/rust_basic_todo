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


#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn remove_command() {
        let command = RemoveCommand::new(vec!["todo".to_string(), "remove".to_string(), "1".to_string()]);
        let exit_code = command.handle();
        assert_eq!(exit_code, 0);
    }
}

use super::Command;
use crate::database;

pub struct ListCommand {
    
}

impl ListCommand {
    pub fn new() -> Self {
        ListCommand {

        }
    }
}

impl Command for ListCommand {
    fn handle(&self) -> i32 {
        let todos = database::read_todos().unwrap();
        for (_, todo) in todos.iter().enumerate() {
            println!("{}. {} {}", todo.id, todo.title, todo.description);
        }
        0
    }
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn list_command() {
        let command = ListCommand::new();
        let exit_code = command.handle();
        assert_eq!(exit_code, 0);
    }
}

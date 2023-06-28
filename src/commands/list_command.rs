use std::fs;

use super::Command;

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
        let contents = fs::read_to_string("storage.txt")
            .expect("Something went wrong reading the file");
        println!("{contents}");
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

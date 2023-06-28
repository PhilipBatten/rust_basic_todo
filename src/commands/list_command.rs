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
        let contents = fs::read_to_string("storage.json")
            .expect("Something went wrong reading the file");
        let parsed_contents = json::parse(&contents).unwrap();

        for (index, item) in parsed_contents.members().enumerate() {
            println!("{} - {}", index + 1, item["description"]);
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

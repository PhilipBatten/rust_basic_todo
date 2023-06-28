use json::object;

use super::Command;
use std::fs;
use std::io::Write;

pub struct AddCommand {
    args: Vec<String>,
}

impl AddCommand {
    pub fn new(args: Vec<String>) -> Self {
        AddCommand {
            args
        }
    }
}

impl Command for AddCommand {
    fn handle(&self) -> i32 {
        let description_option = &self.args.get(2);

        if let Some(description) = description_option {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .append(false)
                .open("storage.json")
                .expect("File not found");
                // .exce/pt("File not found");

            let data = object!{
                description: description.as_str(),
            };

            let contents = fs::read_to_string("storage.json")
                .expect("Something went wrong reading the file");
            let mut parsed_contents = json::parse(&contents).unwrap();

            parsed_contents.push(data).unwrap();

            writeln!(file, "{}", parsed_contents.dump())
                .expect("File write failed");

            println!("todo added!");

            return 0;
        } else {
            println!("description is required");
            return 1;
        }
    }
}


#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn add_command() {
        let command = AddCommand::new(vec!["todo".to_string(), "add".to_string(), "test".to_string()]);
        let exit_code = command.handle();
        assert_eq!(exit_code, 0);
    }
}

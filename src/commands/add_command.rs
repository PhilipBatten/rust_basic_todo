use crate::database;
use super::Command;

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
        let title_option = &self.args.get(2);
        let description_option = &self.args.get(3);

        if let Some(description) = description_option {
            if let Some(title) = title_option {
                database::add_todo(title.to_string(), description.to_string()).unwrap();

                println!("todo added!");
    
                return 0;
            } else {
                println!("title is required");
                return 1;
            }        
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
        let command = AddCommand::new(vec!["todo".to_string(), "add".to_string(), "totle".to_string(), "description".to_string()]);
        let exit_code = command.handle();
        assert_eq!(exit_code, 0);
    }
}

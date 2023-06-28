use std::fs;
use std::io::Write;

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
        let remove_index = &self.args.get(2).unwrap().as_str().parse::<usize>().unwrap();

        let contents = fs::read_to_string("storage.json")
            .expect("Something went wrong reading the file");
        let parsed_contents = json::parse(&contents).unwrap();

        let mut new_content = json::JsonValue::new_array();
        for (index, item) in parsed_contents.members().enumerate() {
            if index+1 != *remove_index {
                new_content.push(item.clone()).unwrap();
            }
        }

        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(false)
            .truncate(true)
            .open("storage.json")
            .expect("File not found");


        // dbg!(new_content.dump());

        write!(file, "{}", new_content.dump())
                .expect("File write failed");
        0
    }
}

use std::env;
use std::process;

mod commands;
use commands::Command;
use commands::add_command::AddCommand;
use commands::list_command::ListCommand;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = args.get(1).unwrap_or_else(|| {
        display_help();
        process::exit(0);
    });

    let exit_code = match command.as_str() {
        "add" => AddCommand::new(args).handle(),
        "list" => ListCommand::new().handle(),
        _ => {
            println!("Unknown Command!");
            0
        },
    };

    dbg!(exit_code);
}

fn display_help() {
    println!("cli application for todos");
    println!("Usage: todo <command> <args>");
    println!();
    println!("available commands:");
    println!("Add - adds a new todo");
    println!("List - gets the list of items");
    println!();
}

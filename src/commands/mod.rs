pub trait Command {
    fn handle(&self) -> i32;
}

// Add command
pub mod add_command;
// List command
pub mod list_command;

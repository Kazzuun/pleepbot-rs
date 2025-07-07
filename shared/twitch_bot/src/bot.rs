use std::collections::HashSet;

use crate::command::Command;

pub struct Bot {
    pub commands: std::collections::HashMap<&'static str, Box<dyn Command>>,
}

impl Bot {
    pub fn new() -> Self {
        Self {
            commands: std::collections::HashMap::new(),
        }
    }

    pub fn register_command(&mut self, command: Box<dyn Command>) {
        let mut names = HashSet::new();
        names.insert(command.name());
        names.extend(command.aliases());

        for name in names.into_iter() {
            // If the key already exists and something is returned, panic
            if self.commands.insert(name, command).is_some() {
                panic!("Clashing command name: {}", name)
            }
        }
    }

    fn error_handler(&self) -> () {

    }

    pub fn on_message() {

    }

    pub fn join() {

    }

    pub fn part() {
        
    }
}

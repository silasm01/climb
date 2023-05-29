pub mod commands;
pub mod objects;
extern crate match_cast;

use commands::Command;
use objects::{Obj, Object};

use std::{
    fmt::Debug,
    io::stdout,
    vec,
};

use crossterm::{execute, terminal::*};
extern crate crossterm;

#[derive(Debug, Default)]
pub struct CLIhandler {
    commands: Vec<Command>,
    objects: Vec<Box<dyn Object>>,
}

impl CLIhandler {
    /// Creates new CLI instance
    /// # Example
    /// ```
    /// let cli = climb::CLIhandler::new();
    /// ```
    pub fn new() -> CLIhandler {
        CLIhandler {
            commands: vec![],
            objects: vec![],
        }
    }

    pub fn add_command(&mut self, trigger: String, function: fn()) {
        self.commands
            .append(&mut vec![Command::new(trigger, function)])
    }

    /// Adds object to current CLI
    /// # Example
    /// ```
    /// let mut cli = climb::CLIhandler::new();
    /// cli.add_object(climb::Objects::percentbar::Percentbar::new(45, "test".to_string()));
    /// ```
    /// Check out the *Objects* module for complete list of objects available
    pub fn add_object(&mut self, obj: Box<dyn Object>) {
        self.objects.append(&mut vec![obj]);
    }

    pub fn parse_args(&mut self) {
        let args: Vec<String> = std::env::args().collect();
        args.iter().for_each(|arg| {
            self.commands.iter().for_each(|trigger| {
                if &trigger.trigger == arg {
                    (trigger.function)()
                }
            })
        })
    }

    pub fn get_object(&mut self, name: &str) -> Option<Obj> {
        for obj in &mut self.objects {
            if let Some(objects) = obj.try_into(name) {
                return Some(objects);
            }
        }
        None
    }

    /// Displays all objects of the CLI
    /// # Example
    /// ```
    /// let mut cli = climb::CLIhandler::new();
    /// cli.add_object(climb::Objects::percentbar::Percentbar::new(45, "test".to_string()));
    /// cli.display()
    /// ```
    pub fn display(&mut self) {
        execute!(stdout(), Clear(crossterm::terminal::ClearType::All)).unwrap();
        self.objects.iter_mut().for_each(|obj| obj.display(0))
    }
}

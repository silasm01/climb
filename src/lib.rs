pub mod commands;
pub mod objects;

use commands::Command;
use objects::{Obj, Object};

use std::{fmt::Debug, io::stdout, vec};

use crossterm::{
    cursor::{Hide, MoveTo, MoveToNextLine, Show},
    execute,
    terminal::*,
};

use crate::objects::InputReturn;
extern crate crossterm;

#[derive(Debug, Default, PartialEq)]
enum Mode {
    #[default]
    Main,
    Alt,
}

#[derive(Debug, Default)]
pub struct CLIhandler {
    commands: Vec<Command>,
    objects: Vec<Box<dyn Object>>,
    pub inputs: Vec<InputReturn>,
    mode: Mode,
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
            inputs: vec![],
            mode: Mode::Main,
        }
    }

    /// Adds command to current CLI
    /// # Example
    /// ```
    /// let mut cli = climb::CLIhandler::new();
    /// cli.add_command("test".to_string(), |cli| {
    ///   cli.add_object(climb::objects::percentbar::Percentbar::new(45, "test".to_string()));
    /// });
    pub fn add_command(&mut self, trigger: String, function: fn(&mut CLIhandler) -> ()) {
        self.commands
            .append(&mut vec![Command::new(trigger, function)])
    }

    /// Adds object to current CLI
    /// # Example
    /// ```
    /// let mut cli = climb::CLIhandler::new();
    /// cli.add_object(climb::objects::percentbar::Percentbar::new(45, "test".to_string()));
    /// ```
    /// Check out the *Objects* module for complete list of objects available
    pub fn add_object(&mut self, obj: Box<dyn Object>) {
        self.objects.append(&mut vec![obj]);
    }

    /// Parses arguments passed to the CLI
    /// # Example
    /// ```
    /// let mut cli = climb::CLIhandler::new();
    /// cli.add_command("test".to_string(), |cli| {
    ///    cli.add_object(climb::objects::percentbar::Percentbar::new(45, "test".to_string()));
    /// });
    /// cli.parse_args();
    /// ```
    pub fn parse_args(&mut self) {
        let args: Vec<String> = std::env::args().collect();
        args.iter().for_each(|arg| {
            let mut commands = self.commands.clone();
            commands.iter_mut().for_each(|trigger| {
                if &trigger.trigger == arg {
                    (trigger.function)(self)
                }
            })
        })
    }

    /// Gets object by name
    /// # Example
    /// ```
    /// let mut cli = climb::CLIhandler::new();
    /// cli.add_object(climb::objects::percentbar::Percentbar::new(45, "test".to_string()));
    /// match cli.get_object("test").unwrap() {
    ///     climb::objects::Obj::Percentbar(obj) => {
    ///        println!("Object found: {}", obj.name)
    ///    }
    ///     climb::objects::Obj::UserInput(obj) => {
    ///       println!("Object found: {}", obj.name)
    ///   }
    ///    _ => {}
    /// }
    /// ```
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
    /// cli.add_object(climb::objects::percentbar::Percentbar::new(45, "test".to_string()));
    /// cli.display()
    /// ```
    pub fn display(&mut self) {
        if self.mode == Mode::Main {
            self.init();
        }

        execute!(stdout(), MoveTo(0, 0)).unwrap();
        let mut temp: Vec<Option<InputReturn>> = vec![];
        self.objects.iter_mut().for_each(|obj| {
            execute!(stdout(), MoveToNextLine(1)).unwrap();
            temp.push(obj.display(0))
        });
        temp.retain(|obj| obj.is_some());
        temp.iter_mut()
            .for_each(|obj| self.inputs.push(obj.clone().unwrap()));
    }

    fn exit(&mut self) {
        self.mode = Mode::Main;
        execute!(stdout(), LeaveAlternateScreen, Show).unwrap();
    }

    fn init(&mut self) {
        self.mode = Mode::Alt;
        execute!(stdout(), EnterAlternateScreen, Hide).unwrap();
    }

    pub fn change_progress<F>(&mut self, name: &str, op: F)
    where
        F: FnOnce(&mut i32),
    {
        match self.get_object(name) {
            Some(Obj::Percentbar(p)) => op(&mut p.progress),
            None => panic!("Object not found"),
            _ => panic!("Object not correct type"),
        }
    }
}

impl Drop for CLIhandler {
    fn drop(&mut self) {
        self.exit();
    }
}

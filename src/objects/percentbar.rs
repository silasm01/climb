use crate::Object;
use crossterm::terminal::*;
use std::{fmt::Debug, println};

use super::{InputReturn, Obj};

/// Displays using percent format.
#[derive(Debug, Clone, Default)]
pub struct Percentbar {
    pub progress: i32,
    pub name: String,
}

impl Object for Percentbar {
    fn display(self: &mut Percentbar, tabs: i32) -> Option<InputReturn> {
        // execute!(stdout(), MoveToNextLine(1),).unwrap();
        let prog = self.progress.clamp(0, 100);
        for _ in 0..tabs {
            print!(" ")
        }
        print!("{} [", self.name);

        let line_len = (size().unwrap().0
            - self.name.len() as u16
            - 5
            - tabs as u16
            - prog.to_string().len() as u16)
            * prog as u16
            / 100
            - 1;

        for _ in 0..line_len {
            print!("-");
        }

        // print!("{}", line_len % 2);

        if line_len % 2 == 0 {
            print!("c");
        } else {
            print!("C");
        }

        for _ in 0..(size().unwrap().0
            - self.name.len() as u16
            - 5
            - tabs as u16
            - prog.to_string().len() as u16)
            * (100 - prog) as u16
            / 100
        {
            print!(" ");
        }

        println!("] {}%", prog);

        return None;
    }

    fn try_into(&mut self, name: &str) -> Option<Obj> {
        if self.name == name {
            return Some(Obj::Percentbar(self));
        }
        None
    }
}

impl Percentbar {
    pub fn new(progress: i32, name: String) -> Box<dyn Object> {
        Box::new(Percentbar { name, progress })
    }
}

/// Adds progress to a percentbar object
/// # Example
/// ```
/// let mut cli = climb::CLIhandler::new();
/// cli.add_object(climb::objects::percentbar::Percentbar::new(45, "test".to_string()));
/// climb::add_progress!(cli, "test", 1);
/// ```
#[macro_export]
macro_rules! add_progress {
    ($cli:expr, $name:expr, $add:expr) => {
        match $cli.get_object($name) {
            Some(climb::objects::Obj::Percentbar(p)) => p.progress += $add,
            None => panic!("Object not found"),
            _ => panic!("Object not correct type"),
        }
    };
}

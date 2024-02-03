use crate::Object;
use crossterm::{cursor::*, execute, terminal::*};
use std::{fmt::Debug, io::stdout, println};

use super::{InputReturn, Obj};

/// Displays using percent format.
#[derive(Debug, Clone, Default)]
pub struct Percentbar {
    pub progress: i32,
    pub name: String,
}

impl Object for Percentbar {
    fn display(self: &mut Percentbar, tabs: i32) -> Option<InputReturn> {
        execute!(stdout(), MoveToNextLine(1),).unwrap();
        for _ in 0..tabs {
            print!(" ")
        }
        print!("{} [", self.name);

        for _ in 0..(size().unwrap().0
            - self.name.len() as u16
            - 5
            - tabs as u16
            - self.progress.to_string().len() as u16)
            * self.progress as u16
            / 100
        {
            print!("-");
        }
        println!("] {}%", self.progress);

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

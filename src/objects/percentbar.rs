use crate::Object;
use crossterm::terminal::*;
use std::{fmt::Debug, println};

use super::{InputReturn, Obj};

#[derive(Debug, Clone, Default)]
pub struct Percentbar {
    pub progress: i32,
    pub name: String,
}

impl Object for Percentbar {
    fn display(self: &mut Percentbar, tabs: i32) -> Option<InputReturn> {
        let prog = self.progress.clamp(0, 100);
        for _ in 0..tabs {
            print!(" ")
        }
        print!("{} [", self.name);

        let line_len = (size().unwrap().0
            - self.name.len() as u16
            - 5
            - 1
            - tabs as u16
            - prog.to_string().len() as u16)
            * prog as u16
            / 100;

        for _ in 0..line_len {
            print!("-");
        }

        if line_len % 2 == 0 {
            print!("C");
        } else {
            print!("c");
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

        None
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

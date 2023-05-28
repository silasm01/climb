use crate::Object;
use crossterm::{cursor::*, execute, terminal::*};
use std::{fmt::Debug, io::stdout, println, vec};

/// Displays using percent format.
#[derive(Debug, Clone)]
pub struct Percentbar {
    pub progress: i32,
    pub name: String,
}

impl Percentbar {
    pub fn new(progress: i32, name: String) -> Object {
        Object::Percentbar(
            Percentbar {
                progress: progress,
                name: name,
            },
            vec![],
        )
    }

    pub(crate) fn display(&self, tabs: i32) {
        execute!(stdout(), MoveToNextLine(1),).unwrap();
        for _ in 0..tabs {
            print!(" ")
        }
        print!("{} [", self.name);

        for _ in 0..(size().unwrap().0 - self.name.len() as u16 - 5 - tabs as u16 - self.progress.to_string().len() as u16)
            * self.progress as u16
            / 100
        {
            print!("-");
        }
        println!("] {}%", self.progress)
    }
}
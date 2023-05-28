use crate::Object;
use crossterm::{cursor::*, execute};
use std::{fmt::Debug, io::stdout, println};
//
#[derive(Debug, Clone)]
pub enum ChoiceStyle {
    HoriList,
    NumList,
}

#[derive(Debug, Clone)]
pub struct UserInput {
    pub name: String,
    pub text: String,
    pub choices: Vec<String>,
    pub input: String,
    pub style: ChoiceStyle
}

impl UserInput {
    pub fn new(name: String, text: String, choices: Vec<String>, style: ChoiceStyle) -> Object {
        Object::UserInput(UserInput {
            name,
            text,
            choices,
            input: "".to_string(),
            style
        })
    }

    pub(crate) fn display(&mut self, tabs: i32) {
        execute!(stdout(), MoveToNextLine(1)).unwrap();
        for _ in 0..tabs {
            print!(" ")
        }
        print!("{} [", self.text);
        self.choices.iter().for_each(|choice| print!("{}/", choice));
        execute!(stdout(), MoveLeft(1)).unwrap();
        println!("]");
        std::io::stdin().read_line(&mut self.input).unwrap();
    }
}
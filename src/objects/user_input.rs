use crate::Object;
use crossterm::{cursor::*, execute};
use std::{fmt::Debug, io::stdout, println};

use super::{InputReturn, Obj};

#[derive(Debug, Clone, Default)]
pub enum ChoiceStyle {
    #[default]
    HoriList,
    NumList,
}

impl Object for UserInput {
    fn display(self: &mut UserInput, tabs: i32) -> Option<InputReturn> {
        for _ in 0..tabs {
            print!(" ")
        }
        match self.style {
            ChoiceStyle::HoriList => {
                print!("{} [", self.text);
                self.choices.iter().for_each(|choice| print!("{}/", choice));
                execute!(stdout(), MoveLeft(1)).unwrap();
                println!("]");
            }
            ChoiceStyle::NumList => {
                println!("{}", self.text);
                self.choices
                    .iter()
                    .enumerate()
                    .for_each(|(i, choice)| println!("{}.{choice}", i + 1))
            }
        }

        if self.input.is_empty() {
            execute!(stdout(), Show).unwrap();

            std::io::stdin().read_line(&mut self.input).unwrap();

            // This keeps asking the user for a valid input if strict input is true
            if self.strict_input
                && !self
                    .clone()
                    .choices
                    .contains(&self.input.trim().to_string())
            {
                self.input = "".to_string();
                println!("Invalid choice. Try again. {}", self.input);
                self.failed_attempts += 1;
                return Some(self.display(tabs).unwrap());
            } else {
                execute!(stdout(), Hide).unwrap();
                return Some(InputReturn {
                    name: self.name.clone(),
                    value: self.input.clone().trim().to_string(),
                });
            }
        } else {
            for _ in 0..self.failed_attempts {
                execute!(stdout(), MoveToNextLine(3)).unwrap();
            }
            print!("{}", self.input);
            return None;
        }
    }

    fn try_into(&mut self, name: &str) -> Option<Obj> {
        if self.name == name {
            return Some(Obj::UserInput(self));
        }
        None
    }
}

#[derive(Debug, Clone, Default)]
pub struct UserInput {
    pub name: String,
    pub text: String,
    pub choices: Vec<String>,
    pub input: String,
    pub style: ChoiceStyle,
    pub strict_input: bool,
    failed_attempts: i32,
}

impl UserInput {
    pub fn new(
        name: String,
        text: String,
        choices: Vec<String>,
        style: ChoiceStyle,
        strict_input: bool,
    ) -> Box<dyn Object> {
        Box::new(UserInput {
            choices,
            input: "".to_string(),
            name,
            style,
            strict_input,
            text,
            failed_attempts: 0,
        })
    }
}

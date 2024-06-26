use std::fmt::Debug;

use self::{percentbar::Percentbar, text::Text, user_input::UserInput};

pub mod percentbar;
pub mod text;
pub mod user_input;

#[derive(Debug)]
pub enum Obj<'a> {
    Percentbar(&'a mut Percentbar),
    UserInput(&'a mut UserInput),
    Text(&'a Text),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InputReturn {
    name: String,
    value: String,
}

pub trait Object: Debug {
    fn display(&mut self, _tabs: i32) -> Option<InputReturn> {
        None
    }

    /// I am about to give up and just want this to work.
    /// It is 2 am and i dont give a fuck anymore
    /// I know this is trash but it works so just let it be
    /// And in 3 months when i return to this project it will be fixed.
    /// UPDATE: Still not fixed :/
    fn try_into(&mut self, _name: &str) -> Option<Obj> {
        None
    }
}

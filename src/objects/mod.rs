use std::fmt::Debug;

use self::{percentbar::Percentbar, user_input::UserInput};

pub mod percentbar;
pub mod user_input;

#[derive(Debug)]
pub enum Obj<'a> {
    Percentbar(&'a mut Percentbar),
    UserInput(&'a mut UserInput)
}

pub trait Object: Debug {
    fn display(&mut self, _tabs: i32) {}

    /// I am about to give up and just want this to work.
    /// It is 2 am and i dont give a fuck anymore
    /// I know this is trash but it works so just let it be
    /// And in 3 months when i return to this project it will be fixed.
    fn try_into(&mut self, _name: &str) -> Option<Obj> {
        None
    }
}
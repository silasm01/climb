use crate::CLIhandler;

#[derive(Debug, Clone)]
pub struct Command {
    pub(crate) trigger: String,
    pub(crate) function: fn(&mut CLIhandler) -> (),
}

impl Command {
    pub fn new(trigger: String, function: fn(&mut CLIhandler) -> ()) -> Command {
        Command { trigger, function }
    }
}

#[derive(Debug, Clone)]
pub struct Command {
    pub(crate) trigger: String,
    pub(crate) function: fn(),
}

impl Command {
    pub fn new(trigger: String, function: fn()) -> Command {
        Command { trigger, function }
    }
}
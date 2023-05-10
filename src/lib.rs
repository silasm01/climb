use std::{vec, fmt::Debug, any::{Any, TypeId}};

#[derive(Debug, Clone)]
pub struct CLIhandler {
    commands: Vec<Command>,
    objects: Vec<Object>,
}

impl CLIhandler {
    pub fn new() -> CLIhandler {
        CLIhandler {
            commands: vec![],
            objects: vec![],
        }
    }

    pub fn add_command(&mut self, trigger: String, function: fn()) {
        self.commands
            .append(&mut vec![Command::new(trigger, function)])
    }

    pub fn add_object(&mut self, obj: Object) -> &mut Object {
        self.objects.append(&mut vec![obj]);
        println!("{:?}",self.objects);
        self.objects.last_mut().unwrap()
    }

    pub fn parse_args(&mut self) {
        let args: Vec<String> = std::env::args().collect();
        args.iter().for_each(|arg| {
            self.commands.iter().for_each(|trigger| {
                if &trigger.trigger == arg {
                    (trigger.function)()
                }
            })
        })
    }
}

#[derive(Debug, Clone)]
pub struct Command {
    trigger: String,
    function: fn(),
}

impl Command {
    fn new(trigger: String, function: fn()) -> Command {
        Command { trigger, function }
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    Statusbar(Statusbar),
    Statusbar2(Statusbar2)
}

pub trait ObjectTrait{}

#[derive(Debug, Clone)]
pub struct Statusbar {
    pub progress: i32,
    pub name: String,
}

impl Statusbar {
    pub fn new(name: String) -> Statusbar {
        Statusbar { progress: 60, name }
    }

    pub fn edit(&mut self) {
        self.progress = 80;
    }
}

#[derive(Debug, Clone)]
pub struct Statusbar2 {
    pub progress: i32,
    pub name: String,
}

impl Statusbar2 {
    pub fn new(name: String) -> Statusbar2 {
        Statusbar2 { progress: 60, name }
    }

    pub fn edit(&mut self) {
        self.progress = 80;
    }
}
use std::{fmt::Debug, io::stdout, println,vec};

use crossterm::{cursor::*, execute, terminal::*};

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

    pub fn add_object(&mut self, obj: Object) {
        self.objects.append(&mut vec![obj]);
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

    pub fn get_object(&mut self, name: String) -> Option<&mut Object> {
        for obj in &mut self.objects {
            if match obj {
                Object::Statusbar(ref mut o) => o.name.clone(),
                Object::Statusbar2(ref mut o) => o.name.clone(),
            } == name {
                println!("{:?}", obj);
                return Some(obj)
            }
        }
        None
    }

    pub fn display(&self) {
        execute!(stdout(), Clear(crossterm::terminal::ClearType::All)).unwrap();
        self.objects.iter().for_each(|obj| obj.display())
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
    Statusbar2(Statusbar2),
}

impl Object {
    fn display(&self) {
        match self {
            Object::Statusbar(o) => o.display(),
            Object::Statusbar2(o) => o.display(),
        }
    }
}

trait ObjectTrait {}

impl ObjectTrait for Statusbar {}
impl ObjectTrait for Statusbar2 {}

#[derive(Debug, Clone)]
pub struct Statusbar {
    pub progress: i32,
    pub name: String,
}

impl Statusbar {
    pub fn new(name: String) -> Statusbar {
        Statusbar { progress: 60, name }
    }

    fn display(&self) {
        execute!(stdout(), MoveToNextLine(1),).unwrap();
        print!("{} [", self.name);

        for _ in 0..(size().unwrap().0 - self.name.len() as u16 - 3) * self.progress as u16 / 100 {
            print!("-");
        }
        println!("]")
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

    fn display(&self) {
        execute!(stdout(), MoveToNextLine(1),).unwrap();
        print!("{} [", self.name);

        for _ in 0..(size().unwrap().0 - self.name.len() as u16 - 3) * self.progress as u16 / 100 {
            print!("-");
        }
        println!("]")
    }
}

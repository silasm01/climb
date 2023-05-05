use std::vec;

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

    pub fn add_object(&mut self, object: Object) {
        self.objects.append(&mut vec![object.clone()]);
    }

    pub fn get_object(&mut self, name: String) -> Option<&mut Object>{
        for obj in &mut self.objects {
            let o = match obj {
                Object::Statusbar(ref mut o) => o,
            };
            if o.name == name {
                return Some(obj);
            }
        }
        None
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
}

impl Object {
    pub fn edit(&mut self) {
        match self {
            Object::Statusbar(obj) => obj.edit(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Statusbar {
    pub progress: i32,
    name: String,
}

impl Statusbar {
    pub fn new(name: String) -> Statusbar {
        Statusbar { progress: 60, name }
    }

    pub fn edit(&mut self) {
        self.progress = 80;
    }
}

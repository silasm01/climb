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

    pub fn edit_object(&mut self){
        for obj in &mut self.objects {
            match obj {
                Object::Statusbar(ref mut o) => o.progress = 80,
            }
        }
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

#[derive(Debug, Clone)]
pub struct Statusbar {
    pub progress: i32,
    name: String,
}

impl Statusbar {
    pub fn new(name: String) -> Statusbar {
        Statusbar { progress: 60, name }
    }
}

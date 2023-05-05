use std::vec;

#[derive(Debug)]
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

    pub fn get_object(&self, name: String) -> Option<Object>{
        for obj in &self.objects {
            let o = match &obj {
                    Object::Statusbar(e) => e,
                };
            if o.name == name {
                return Some(Object::Statusbar(o.clone()));
            }
        };
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

#[derive(Debug)]
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
    pub fn edit(self, progress: i32, name: String) {
        //let t = self.clone();
        match self {
            Object::Statusbar(mut obj) => obj.edit(progress, name), //obj.name = "test3".to_string()
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

    fn edit(mut self, progress: i32, name: String) {
        self.progress = progress;
        self.name = name;
        println!("k,, {:?}", self)
    }
}

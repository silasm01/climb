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

    pub fn add_object(&mut self, object: Object) {
        self.objects.append(&mut vec![object.clone()]);
    }

    pub fn get_object(&mut self, name: String) -> Option<&mut dyn ObjectTrait>
    {
        for obj in &mut self.objects {
            let o: &mut dyn ObjectTrait = match obj {
                Object::Statusbar(ref mut o) => o,
                Object::Statusbar2(ref mut e) => e,
            };
            //println!("oty: {:?}, {:?}", o.as_any().type_id(), o.as_any_mut().downcast_mut::<Statusbar>().unwrap().name);
            match o.as_any().type_id() {
                TypeId::of::<Statusbar>() => (),
                TypeId::of::<Statusbar2>() => (),
                _ => ()
            }
            // if o.as_any().type_id() == TypeId::of::<Statusbar>() {
            //     if o.as_any_mut().downcast_mut::<Statusbar>().unwrap().name == name {
            //         return Some(o.as_any_mut().downcast_mut::<Statusbar2>().unwrap());
            //     }
            // }
            // if o.as_any().type_id() == TypeId::of::<Statusbar2>() {
            //     println!("{:?}, {:?}", o.as_any_mut().downcast_mut::<Statusbar2>().unwrap().name, name);
            //     if o.as_any_mut().downcast_mut::<Statusbar2>().unwrap().name == name {
            //         println!("worked");
            //         return Some(o.as_any_mut().downcast_mut::<Statusbar2>().unwrap());
            //     }
            // }
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
    Statusbar2(Statusbar2)
}

pub trait ObjectTrait: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl ObjectTrait for Statusbar {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
impl ObjectTrait for Statusbar2 {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

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

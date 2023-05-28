use self::{percentbar::Percentbar, user_input::UserInput};

pub mod percentbar;
pub mod user_input;
//
#[derive(Debug, Clone)]
pub enum Object {
    Percentbar(Percentbar, Vec<Object>),
    UserInput(UserInput),
}

impl Object {
    pub(crate) fn display(&mut self, tabs: i32) {
        match self {
            Object::Percentbar(o, i) => {
                o.display(tabs);
                for obj in i {
                    obj.display(tabs + 1)
                }
            }
            Object::UserInput(o) => o.display(tabs),
        }
    }

    pub fn add_child_object(&mut self, obj: Object) {
        match self {
            Object::Percentbar(_, ref mut l) => l.append(&mut vec![obj]),
            Object::UserInput(_) => todo!(),
        }
    }

    pub fn get_child(&mut self, name: String) -> Option<&mut Object> {
        match self {
            Object::Percentbar(_, objects) => {
                for obj in objects {
                    match obj {
                        Object::Percentbar(o, _) => {
                            if o.name == name {
                                return Some(obj);
                            }
                        }
                        Object::UserInput(_) => todo!(),
                    }
                }
            }
            Object::UserInput(_) => todo!(),
        }
        None
    }
}
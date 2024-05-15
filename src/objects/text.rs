use super::{InputReturn, Obj, Object};

#[derive(Debug, Clone)]
pub struct Text {
    name: String,
    pub(crate) text: String,
}

impl Object for Text {
    fn display(&mut self, _tabs: i32) -> Option<InputReturn> {
        println!("{}", self.text);
        None
    }

    fn try_into(&mut self, name: &str) -> Option<Obj> {
        if self.name == name {
            return Some(Obj::Text(self));
        }
        None
    }
}

impl Text {
    pub fn new(name: String, text: String) -> Box<dyn Object> {
        Box::new(Text { name, text })
    }
}

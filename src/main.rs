use std::any::Any;

use climb::{Object, Statusbar, Statusbar2};

mod lib;
fn main() {
    let mut cli = climb::CLIhandler::new();
    cli.add_command("tres".to_string(), test);
    let t = cli.add_object(Object::Statusbar(Statusbar::new("test".to_string())));
    match t {
        Object::Statusbar(ref mut o) => o.progress = 100,
        Object::Statusbar2(_) => todo!(),
    }

    let u = cli.add_object(Object::Statusbar2(Statusbar2::new("test2".to_string())));
}

fn test() {
    println!("Works")
}
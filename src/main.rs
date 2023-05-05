use std::any::Any;

use climb::{Object, Statusbar};

mod lib;
fn main() {
    let mut cli = climb::CLIhandler::new();
    cli.add_command("tres".to_string(), test);
    cli.add_object(Object::Statusbar(Statusbar::new("test".to_string())));
    println!("{:?}", cli);
    cli.parse_args();
    cli.get_object("test".to_string()).unwrap().edit(70, "test2".to_string());
    println!("fin: {:?}", cli);
}

fn test() {
    println!("Works")
}
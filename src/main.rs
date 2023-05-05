use std::any::Any;

use climb::{Object, Statusbar};

mod lib;
fn main() {
    let mut cli = climb::CLIhandler::new();
    cli.add_command("tres".to_string(), test);
    cli.add_object(Object::Statusbar(Statusbar::new("test".to_string())));
    println!("{:?}", cli);
    cli.parse_args();
    cli.edit_object();
    println!("fin: {:?}", cli);
}

fn test() {
    println!("Works")
}
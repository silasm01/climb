use std::any::Any;

use climb::{Object, Statusbar, Statusbar2};

mod lib;
fn main() {
    let mut cli = climb::CLIhandler::new();
    cli.add_command("tres".to_string(), test);
    cli.add_object(Object::Statusbar(Statusbar::new("test".to_string())));
    cli.add_object(Object::Statusbar2(Statusbar2::new("test32".to_string())));
    println!("{:?}", cli);
    cli.parse_args();
    println!("{:?}", cli.get_object("test32".to_string()).unwrap() = 70);
    println!("fin: {:?}", cli);
}

fn test() {
    println!("Works")
}
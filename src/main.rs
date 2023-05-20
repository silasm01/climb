use std::{thread, time::Duration};

use climb::{Object, Statusbar, Statusbar2};

mod lib;
fn main() {
    let mut cli = climb::CLIhandler::new();
    cli.add_command("tres".to_string(), test);
    cli.parse_args();
    cli.add_object(Object::Statusbar(Statusbar::new("test".to_string())));
    cli.display();
    thread::sleep(Duration::from_millis(1000));
    match cli.get_object("test".to_string()).unwrap() {
        Object::Statusbar(ref mut o) => o.progress = 100,
        Object::Statusbar2(_) => todo!(),
    }
    cli.display();
    thread::sleep(Duration::from_millis(1000));
    cli.add_object(Object::Statusbar2(Statusbar2::new("test2".to_string())));
    cli.display();
    thread::sleep(Duration::from_millis(1000));
    match cli.get_object("test2".to_string()).unwrap() {
        Object::Statusbar2(ref mut o) => o.progress = 20,
        Object::Statusbar(_) => todo!(),
    }
    cli.display();
    while true {
        thread::sleep(Duration::from_millis(200));
        match cli.get_object("test2".to_string()).unwrap() {
            Object::Statusbar2(ref mut o) => o.progress += 1,
            Object::Statusbar(_) => todo!(),
        }
        cli.display();
    }
}

fn test() {
    println!("Works")
}

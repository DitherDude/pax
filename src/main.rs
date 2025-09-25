use std::env;

use paxr::{Command, Flag};

fn main() {
    let args: Vec<String> = env::args().collect();
    let help_flag = Flag {
        short: 'h',
        long: String::from("help"),
        about: String::from("help for pax"),
        func: Box::new(help_work),
    };
    let command = Command {
        name: String::from("pax"),
        about: String::new(),
        version: String::from("PAX is the official package manager for the Oreon 11."),
        flags: vec![help_flag],
        subcommands: Vec::new(),
        func: Box::new(main_work),
        man: String::from("There is no manual. 'Go' sucks."),
    };
    command.run(&args);
}

fn main_work(_cmd: &Command, _args: &[String]) {
    println!("Hello, World!");
}

fn help_work(parent: &Command) {
    println!("{}", parent.help());
}

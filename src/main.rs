use std::env;

use paxr::{Command, Flag, StateBox};

fn main() {
    let args: Vec<String> = env::args().collect();
    let sample_flag = Flag {
        short: 's',
        long: String::from("sample"),
        about: String::from("Does nothing"),
        consumer: false,
        breakpoint: false,
        func: help_work,
    };
    let mut command = Command {
        name: String::from("pax"),
        about: String::new(),
        version: String::from("PAX is the official package manager for the Oreon 11."),
        flags: vec![sample_flag],
        subcommands: Vec::new(),
        states: StateBox::new(),
        func: main_work,
        man: String::from("There is no manual. 'Go' sucks."),
    };
    command.run(&args);
}

fn main_work(_cmd: &mut Command, _args: &[String]) {
    println!("Hello, World!\n{}", _cmd.states.len());
}

fn help_work(_parent: &mut StateBox) {
    println!("Did nothing successfully.");
}

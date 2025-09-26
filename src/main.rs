use std::env;

use paxr::{Command, Flag, StateBox};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let sample_flag = Flag {
        short: 's',
        long: String::from("sample"),
        about: String::from("Does nothing"),
        consumer: false,
        breakpoint: false,
        run_func: sample_work,
    };
    let consumable_flag = Flag {
        short: 'c',
        long: String::from("consumable"),
        about: String::from("Consumes the next arg"),
        consumer: true,
        breakpoint: false,
        run_func: consumable_work,
    };
    let mut command = Command {
        name: String::from("pax"),
        about: String::new(),
        version: String::from("PAX is the official package manager for the Oreon 11."),
        flags: vec![sample_flag, consumable_flag],
        subcommands: Vec::new(),
        states: StateBox::new(),
        run_func: main_work,
        man: String::from("There is no manual. 'Go' sucks."),
    };
    command.run(args.iter());
}

fn main_work(states: &StateBox) {
    println!("Hello, World!\n{}", states.len());
}

fn sample_work(_parent: &mut StateBox, _flag: Option<&String>) {
    println!("Did nothing successfully.");
}

fn consumable_work(_parent: &mut StateBox, flag: Option<&String>) {
    println!("Got flag {flag:?}!");
}

use std::{any::Any, collections::HashMap};

pub struct Command {
    pub name: String,
    pub about: String,
    pub version: String,
    pub flags: Vec<Flag>,
    pub subcommands: Vec<Command>,
    pub states: StateBox,
    pub func: fn(cmd: &mut Command, args: &[String]),
    pub man: String,
}

impl PartialEq for Command {
    fn eq(
        &self,
        Command {
            name: _,
            about: _,
            version: _,
            flags: _,
            subcommands: _,
            states: _,
            func: _,
            man: _,
        }: &Self,
    ) -> bool {
        false
    }
}

impl Command {
    pub fn help(&self) -> String {
        let mut help = String::new();
        help.push_str(&format!("{}\n", self.version));
        let mut commands = String::new();
        let mut attrs = String::from(&format!("Usage:\n  {} [flags]\n", self.name));
        let mut flags = String::from("\nFlags:\n");
        for flag in &self.flags {
            flags.push_str(&format!("  {}\n", flag.help()));
        }
        flags.push_str(&format!("  -h, --help\thelp for {}\n", self.name));
        if self.subcommands != Vec::new() {
            attrs.push_str(&format!("  {} [command]\n", self.name));
            commands = String::from("\nAvailable Commands:\n");
            for command in &self.subcommands {
                commands.push_str(&format!("  {}\n", command.micro_help()));
            }
        }
        help.push_str(&format!("{attrs}{commands}{flags}\n"));
        help.push_str(&format!(
            "Use {} [command] --help for more information about a command.",
            self.name
        ));
        help
    }
    fn micro_help(&self) -> String {
        let mut help = String::new();
        help.push_str(&format!("{}\t{}", self.name, self.about));
        help
    }
    pub fn run(&mut self, args: &[String]) {
        let mut args_iter = args.iter();
        let mut opr = None;
        'outer: while let Some(arg) = args_iter.nth(0) {
            if let Some(l_arg) = arg.strip_prefix("--") {
                match l_arg {
                    "help" => {
                        println!("{}", self.help());
                        return;
                    }
                    _ => {
                        for flag in &self.flags {
                            if flag.long == l_arg {
                                let val = if flag.consumer {
                                    args_iter.nth(0)
                                } else {
                                    None
                                };
                                if flag.breakpoint {
                                    if opr.is_some() {
                                        panic!("Multiple breakpoint arguments supplied!");
                                    }
                                    opr = Some((flag, val));
                                } else {
                                    (flag.func)(&mut self.states, val)
                                }
                                continue 'outer;
                            }
                        }
                        let error = format!("unknown flag: '{l_arg}'");
                        println!("Error: {error}\n{}\n\n{error}", self.help());
                        return;
                    }
                }
            } else if let Some(s_arg) = arg.strip_prefix("-") {
                'mid: for chr in s_arg.chars() {
                    match chr {
                        'h' => {
                            println!("{}", self.help());
                            return;
                        }
                        c => {
                            for flag in &self.flags {
                                if flag.short == c {
                                    let val = if flag.consumer {
                                        args_iter.nth(0)
                                    } else {
                                        None
                                    };
                                    if flag.breakpoint {
                                        if opr.is_some() {
                                            panic!("Multiple breakpoint arguments supplied!");
                                        }
                                        opr = Some((flag, val));
                                    } else {
                                        (flag.func)(&mut self.states, val)
                                    }
                                    continue 'mid;
                                }
                            }
                            let error = format!("unknown shorthand flag: '{c}' in -{s_arg}");
                            println!("Error: {error}\n{}\n\n{error}", self.help());
                            return;
                        }
                    }
                }
            }
        }
        if let Some((opr, val)) = opr {
            (opr.func)(&mut self.states, val)
        } else {
            (self.func)(self, args)
        }
    }
}

pub struct Flag {
    pub short: char,
    pub long: String,
    pub about: String,
    pub consumer: bool,
    pub breakpoint: bool,
    pub func: fn(parent: &mut StateBox, flag: Option<&String>),
}

impl PartialEq for Flag {
    fn eq(
        &self,
        Flag {
            short: _,
            long: _,
            about: _,
            consumer: _,
            breakpoint: _,
            func: _,
        }: &Self,
    ) -> bool {
        false
    }
}

impl Flag {
    pub fn help(&self) -> String {
        let mut help = String::new();
        help.push_str(&format!("-{}, --{}\t{}", self.short, self.long, self.about));
        help
    }
    // pub fn run(&self, states: &mut StateBox) {
    //     (self.func)(states)
    // }
}

pub struct StateBox {
    store: HashMap<&'static str, Box<dyn Any>>,
}

impl StateBox {
    pub fn new() -> Self {
        StateBox {
            store: HashMap::new(),
        }
    }
    pub fn insert<T: 'static>(&mut self, key: &'static str, value: T) -> Result<(), String> {
        if self.store.contains_key(key) {
            return Err(String::from(
                "Key already exists! If you wish to update this value, use `set()` method instead.",
            ));
        }
        self.store.insert(key, Box::new(value));
        Ok(())
    }
    pub fn remove(&mut self, key: &str) -> Result<(), String> {
        match self.store.remove_entry(key) {
            Some(_) => Ok(()),
            None => Err(String::from("Cannot remove nonexistant key!")),
        }
    }
    pub fn get<T: 'static>(&self, key: &str) -> Option<&T> {
        self.store.get(key)?.downcast_ref::<T>()
    }
    pub fn set<T: 'static>(&mut self, key: &str, value: T) -> Result<(), String> {
        if let Some(state) = self.store.get_mut(key) {
            *state = Box::new(value);
            Ok(())
        } else {
            Err(String::from(
                "Key not found. If you wish to create this value, use `insert()` method instead.",
            ))
        }
    }
    pub fn push<T: 'static>(&mut self, _key: &str, _value: T) -> ! {
        //Learned the '!' (bang) return type from RUst Kernel dev ;P
        unimplemented!()
    }
    pub fn pop<T: 'static>(&mut self, key: &str) -> Option<T> {
        self.store
            .remove(key)?
            .downcast::<T>()
            .map(|x| Some(*x))
            .ok()?
    }
    pub fn shove<T: 'static>(&mut self, key: &'static str, value: T) {
        if let Some(state) = self.store.get_mut(key) {
            *state = Box::new(value)
        } else {
            self.store.insert(key, Box::new(value));
        }
    }
    pub fn yank(&mut self, key: &str) {
        // WARNING: This function has VERY different connotation to the 'yank' from NVIM!
        self.store.remove(key);
    }
    pub fn len(&self) -> usize {
        self.store.len()
    }
    // This is to make Clippy happy
    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
}

impl Default for StateBox {
    fn default() -> Self {
        Self::new()
    }
}

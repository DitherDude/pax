pub struct Command {
    pub name: String,
    pub about: String,
    pub version: String,
    pub flags: Vec<Flag>,
    pub subcommands: Vec<Command>,
    pub func: Box<dyn Commandable>,
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
        let mut has_attrs = false;
        let mut attrs = String::new();
        let mut flags = String::new();
        let mut commands = String::new();
        if self.flags != Vec::new() {
            has_attrs = true;
            attrs = String::from(&format!("Usage:\n  {} [flags]\n", self.name));
            flags = String::from("\nFlags:\n");
            for flag in &self.flags {
                flags.push_str(&format!("  {}\n", flag.help()));
            }
        }
        if self.subcommands != Vec::new() {
            if has_attrs {
                attrs.push_str(&format!("  {} [command]\n", self.name));
            } else {
                attrs = String::from(&format!("Usage:\n  {} [command]\n", self.name))
            }
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
    pub fn run(&self, args: &[String]) {
        let mut opr = None;
        for arg in args {
            if let Some(l_arg) = arg.strip_prefix("--") {
                for flag in &self.flags {
                    if flag.long == l_arg {
                        opr = Some(flag);
                        break;
                    }
                }
            } else if let Some(mut s_arg) = arg.strip_prefix("-") {
                'mid: for chr in s_arg.chars() {
                    for flag in &self.flags {
                        if flag.short == chr {
                            opr = Some(flag);
                            s_arg = &s_arg[1..];
                            continue 'mid;
                        }
                    }
                    let error = format!("unknown shorthand flag: '{chr}' in -{s_arg}");
                    println!("Error: {error}\n{}\n\n{error}", self.help());
                    return;
                }
            }
        }
        if let Some(flag) = opr {
            flag.run(self);
        } else {
            self.func.run(self, args);
        }
    }
}

pub trait Commandable {
    fn run(&self, cmd: &Command, args: &[String]);
}

impl<T: Fn(&Command, &[String])> Commandable for T {
    fn run(&self, cmd: &Command, args: &[String]) {
        self(cmd, args);
    }
}

pub struct Flag {
    pub short: char,
    pub long: String,
    pub about: String,
    pub func: Box<dyn Flaggable>,
}

impl PartialEq for Flag {
    fn eq(
        &self,
        Flag {
            short: _,
            long: _,
            about: _,
            func: _,
        }: &Self,
    ) -> bool {
        false
    }
}

pub trait Flaggable {
    fn run(&self, parent: &Command);
}

impl<T: Fn(&Command)> Flaggable for T {
    fn run(&self, parent: &Command) {
        self(parent);
    }
}

impl Flag {
    pub fn help(&self) -> String {
        let mut help = String::new();
        help.push_str(&format!(
            "-{}, --{}   {}",
            self.short, self.long, self.about
        ));
        help
    }
    pub fn run(&self, parent: &Command) {
        self.func.run(parent);
    }
}

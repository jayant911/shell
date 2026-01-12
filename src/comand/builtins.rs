use pathsearch::find_executable_in_path;
use std::env;

use super::BUILT_IN_COMMANDS;
use super::builtin::Builtin;

//exit: it can take 0 or 1 arguments as exit code
pub struct ExitComand {
    pub args: Vec<String>,
}

impl Builtin for ExitComand {
    fn execute(&self) {
        if self.args.len() > 1 {
            println!("exit: too many arguments");
        } else {
            std::process::exit(
                self.args
                    .first()
                    .and_then(|s| s.parse::<i32>().ok())
                    .unwrap_or(0),
            );
        }
    }
}

//echo: take 0 or multiple arguments and pnd print them
pub struct EchoComand {
    pub text: String,
}

impl Builtin for EchoComand {
    fn execute(&self) {
        println!("{}", self.text);
    }
}

//type: it take 1 or multiple argument as command name and tell about them
pub struct TypeComand {
    pub args: Vec<String>,
}

impl Builtin for TypeComand {
    fn execute(&self) {
        for arg in self.args.iter() {
            if BUILT_IN_COMMANDS.contains(&arg.as_str()) {
                // 1. Check built_in_command list.
                println!("{} is a shell builtin", arg);
            } else if let Some(path) = find_executable_in_path(arg) {
                // 2. Check PATH.
                // TODO: impliment the PATH searching features using std library.
                println!("{} is {}", arg, path.display());
            } else {
                println!("{}: not found", arg);
            }
        }
    }
}

pub struct PwdComand;

impl Builtin for PwdComand {
    fn execute(&self) {
        let pwd = env::current_dir();
        match pwd {
            Ok(directory_path) => println!("{}", directory_path.display()),
            Err(e) => eprintln!("error geting current directory: {}", e),
        }
    }
}

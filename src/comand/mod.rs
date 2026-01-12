pub mod builtin;
pub mod builtins;

use builtin::Builtin;

use builtins::EchoComand;
use builtins::ExitComand;
use builtins::NotFound;
use builtins::TypeComand;

// Lis of all the known command.
pub const BUILT_IN_COMMANDS: [&str; 3] = ["exit", "echo", "type"];

pub enum Command {
    ExitCmd(ExitComand),
    EchoCmd(EchoComand),
    TypeCmd(TypeComand),
    CmdNotFound(NotFound),
}

impl Command {
    // Parse the command into a varient of Command enum.
    pub fn parse(input: &str) -> Self {
        let mut input = input.split_whitespace();
        let cmd = match input.next() {
            Some(comand) => comand,
            None => {
                return Command::CmdNotFound(NotFound {
                    name: String::new(),
                });
            }
        };

        let args: Vec<String> = input.map(|s| s.to_string()).collect();

        // Match the command and execute coresponding command
        match cmd {
            "exit" => Command::ExitCmd(ExitComand { args }),
            "echo" => Command::EchoCmd(EchoComand {
                text: args.join(" "),
            }),
            "type" => Command::TypeCmd(TypeComand { args }),

            other => Command::CmdNotFound(NotFound {
                name: other.to_string(),
            }),
        }
    }

    // Execute the coresponding command.
    pub fn execute(self) {
        match self {
            Command::ExitCmd(cmd) => cmd.execute(),
            Command::EchoCmd(cmd) => cmd.execute(),
            Command::TypeCmd(cmd) => cmd.execute(),
            Command::CmdNotFound(cmd) => cmd.execute(),
        }
    }
}

pub mod builtin;
pub mod builtins;
pub mod executable;

use builtin::Builtin;

use builtins::EchoComand;
use builtins::ExitComand;
use builtins::PwdComand;
use builtins::TypeComand;
use executable::Executable;

// Lis of all the known command.
pub const BUILT_IN_COMMANDS: [&str; 3] = ["exit", "echo", "type"];

pub enum Command {
    ExitCmd(ExitComand),
    EchoCmd(EchoComand),
    TypeCmd(TypeComand),
    PWDCmd(PwdComand),
    OtherCmd(Executable),
    CmdNotFound,
}

impl Command {
    // Parse the command into a varient of Command enum.
    pub fn parse(input: &str) -> Self {
        let mut input = input.split_whitespace();
        // Check if the command is empty.
        let cmd = match input.next() {
            Some(comand) => comand,
            None => {
                return Command::CmdNotFound;
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
            "pwd" => Command::PWDCmd(PwdComand),
            other => Command::OtherCmd(Executable {
                name: other.to_string(),
                args,
            }),
        }
    }

    // Execute the coresponding command.
    pub fn execute(self) {
        match self {
            Command::ExitCmd(cmd) => cmd.execute(),
            Command::EchoCmd(cmd) => cmd.execute(),
            Command::TypeCmd(cmd) => cmd.execute(),
            Command::PWDCmd(cmd) => cmd.execute(),
            Command::OtherCmd(cmd) => cmd.execute(),
            //This line never going to call because it for handling empty user input
            Command::CmdNotFound => println!("command not found"),
        }
    }
}

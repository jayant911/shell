#[allow(unused_imports)]
use std::io::{self, Write};

mod comand;

use comand::Command;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        let user_input = user_input.trim();
        if user_input.is_empty() {
            continue;
        } else {
            let command = Command::parse(user_input);
            command.execute();
        }
    }
}


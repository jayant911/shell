#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut command = String::new();
        io::stdin().read_line(&mut command);

        let command = command.trim();
        if command.is_empty() {
            continue;
        }
        println!("{}: command not found", command.trim());
    }
}

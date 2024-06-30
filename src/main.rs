use std::io::{self, Write};
use std::process;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let command = input.trim();

        if command.is_empty() {
            continue;
        } else if command.starts_with("exit ") {
            let parts: Vec<&str> = command.split_whitespace().collect();
            if parts.len() == 2 {
                if let Ok(code) = parts[1].parse::<i32>() {
                    process::exit(code);
                }
            }
            println!("Invalid exit command format");
        } else if command.starts_with("echo ") {
            let echo_output = &command[5..];
            println!("{}", echo_output);
        } else {
            println!("{}: command not found", command);
        }
    }
}

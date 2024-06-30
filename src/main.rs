use std::io::{self, Write};
use std::process;

const BUILTINS: [&str; 3] = ["echo", "exit", "type"];

fn main() {
    loop {
        print_prompt();

        let command = read_command();

        if command.is_empty() {
            continue;
        }

        match &command[..] {
            cmd if cmd.starts_with("exit ") => handle_exit_command(cmd),
            cmd if cmd.starts_with("echo ") => handle_echo_command(cmd),
            cmd if cmd.starts_with("type ") => handle_type_command(cmd),
            _ => handle_unknown_command(command),
        }
    }
}

fn print_prompt() {
    print!("$ ");
    io::stdout().flush().expect("Failed to flush stdout");
}

fn read_command() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn handle_exit_command(command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() == 2 {
        if let Ok(code) = parts[1].parse::<i32>() {
            process::exit(code);
        }
    }
    println!("Invalid exit command format");
}

fn handle_echo_command(command: &str) {
    let echo_output = &command[5..];
    println!("{}", echo_output);
}

fn handle_type_command(command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() == 2 {
        let cmd_to_check = parts[1];
        if BUILTINS.contains(&cmd_to_check) {
            println!("{} is a shell builtin", cmd_to_check);
        } else {
            println!("{}: not found", cmd_to_check);
        }
    } else {
        println!("Invalid type command format");
    }
}

fn handle_unknown_command(command: String) {
    println!("{}: command not found", command);
}

use std::env;
use std::io::{self, Write};

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
            _ => execute_command(&command),
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
            std::process::exit(code);
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
            return;
        }

        if let Some(path) = find_executable_in_path(cmd_to_check) {
            println!("{} is {}", cmd_to_check, path.display());
        } else {
            println!("{}: not found", cmd_to_check);
        }
    } else {
        println!("Invalid type command format");
    }
}

fn execute_command(command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    let program = parts[0];

    if BUILTINS.contains(&program) {
        println!("{} is a shell builtin", program);
        return;
    }

    if let Some(path) = find_executable_in_path(program) {
        let status = execute_external_command(&path, &parts[1..]);
        if let Some(status) = status {
            if !status.success() {
                println!("Command failed with status: {}", status);
            }
        } else {
            println!("Failed to execute command");
        }
    } else {
        println!("{}: not found", program);
    }
}

fn find_executable_in_path(command: &str) -> Option<std::path::PathBuf> {
    if let Some(paths) = env::var_os("PATH") {
        for path in env::split_paths(&paths) {
            let exe_path = path.join(command);
            if exe_path.is_file() && can_execute(&exe_path) {
                return Some(exe_path);
            }
        }
    }
    None
}

fn can_execute(path: &std::path::Path) -> bool {
    use std::os::unix::fs::PermissionsExt;

    if let Ok(metadata) = path.metadata() {
        let permissions = metadata.permissions();
        permissions.mode() & 0o111 != 0
    } else {
        false
    }
}

fn execute_external_command(
    program: &std::path::Path,
    args: &[&str],
) -> Option<std::process::ExitStatus> {
    let status = std::process::Command::new(program)
        .args(args)
        .status()
        .expect("Failed to execute command");

    Some(status)
}

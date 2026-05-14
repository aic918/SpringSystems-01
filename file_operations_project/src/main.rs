use std::io::{self, Write};
use std::process::Command;

enum FileOperation {
    List(String),
    Display(String),
    Create(String, String),
    Remove(String),
    Pwd,
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(path) => {
            let output = Command::new("ls")
                .arg(path)
                .output();

            match output {
                Ok(result) => {
                    if result.status.success() {
                        println!("{}", String::from_utf8_lossy(&result.stdout));
                    } else {
                        println!("Error: {}", String::from_utf8_lossy(&result.stderr));
                    }
                }
                Err(e) => println!("Failed to run ls command: {}", e),
            }
        }

        FileOperation::Display(file_path) => {
            let output = Command::new("cat")
                .arg(file_path)
                .output();

            match output {
                Ok(result) => {
                    if result.status.success() {
                        println!("{}", String::from_utf8_lossy(&result.stdout));
                    } else {
                        println!("Error: {}", String::from_utf8_lossy(&result.stderr));
                    }
                }
                Err(e) => println!("Failed to run cat command: {}", e),
            }
        }

        FileOperation::Create(file_path, content) => {
            let output = Command::new("sh")
                .arg("-c")
                .arg("printf '%s' \"$1\" > \"$0\"")
                .arg(&file_path)
                .arg(&content)
                .output();

            match output {
                Ok(result) => {
                    if result.status.success() {
                        println!("File '{}' created successfully.", file_path);
                    } else {
                        println!("Error: {}", String::from_utf8_lossy(&result.stderr));
                    }
                }
                Err(e) => println!("Failed to create file: {}", e),
            }
        }

        FileOperation::Remove(file_path) => {
            let output = Command::new("rm")
                .arg(&file_path)
                .output();

            match output {
                Ok(result) => {
                    if result.status.success() {
                        println!("File '{}' removed successfully.", file_path);
                    } else {
                        println!("Error: {}", String::from_utf8_lossy(&result.stderr));
                    }
                }
                Err(e) => println!("Failed to remove file: {}", e),
            }
        }

        FileOperation::Pwd => {
            let output = Command::new("pwd")
                .output();

            match output {
                Ok(result) => {
                    if result.status.success() {
                        print!("Current working directory: ");
                        println!("{}", String::from_utf8_lossy(&result.stdout));
                    } else {
                        println!("Error: {}", String::from_utf8_lossy(&result.stderr));
                    }
                }
                Err(e) => println!("Failed to run pwd command: {}", e),
            }
        }
    }
}

fn main() {
    println!("Welcome to the File Operations Program!");

    loop {
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");

        let choice = get_input("\nEnter your choice (0-5): ");

        match choice.as_str() {
            "1" => {
                let path = get_input("Enter directory path: ");
                let operation = FileOperation::List(path);
                perform_operation(operation);
            }

            "2" => {
                let file_path = get_input("Enter file path: ");
                let operation = FileOperation::Display(file_path);
                perform_operation(operation);
            }

            "3" => {
                let file_path = get_input("Enter file path: ");
                let content = get_input("Enter content: ");
                let operation = FileOperation::Create(file_path, content);
                perform_operation(operation);
            }

            "4" => {
                let file_path = get_input("Enter file path: ");
                let operation = FileOperation::Remove(file_path);
                perform_operation(operation);
            }

            "5" => {
                let operation = FileOperation::Pwd;
                perform_operation(operation);
            }

            "0" => {
                println!("\nGoodbye!");
                break;
            }

            _ => {
                println!("Invalid choice. Please enter a number from 0 to 5.");
            }
        }
    }
}
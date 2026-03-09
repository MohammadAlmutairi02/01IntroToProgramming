use std::io::{self, Write};
use std::process::Command;

// Enum representing file operations
enum FileOperation {
    List(String),
    Display(String),
    Create(String, String),
    Remove(String),
    Pwd,
}

// Function to perform operations
fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(directory) => {
            let status = Command::new("ls")
                .arg(directory)
                .status()
                .expect("Failed to execute ls");

            if !status.success() {
                println!("Error listing directory");
            }
        }

        FileOperation::Display(file) => {
            let status = Command::new("cat")
                .arg(file)
                .status()
                .expect("Failed to execute cat");

            if !status.success() {
                println!("Error displaying file");
            }
        }

        FileOperation::Create(file_path, content) => {
            let command = format!("echo '{}' > {}", content, file_path);

            let output = Command::new("sh")
                .arg("-c")
                .arg(command)
                .status()
                .expect("Failed to create file");

            if output.success() {
                println!("File '{}' created successfully.", file_path);
            } else {
                println!("Failed to create file.");
            }
        }

        FileOperation::Remove(file_path) => {
            let status = Command::new("rm")
                .arg(&file_path)
                .status()
                .expect("Failed to execute rm");

            if status.success() {
                println!("File '{}' removed successfully.", file_path);
            } else {
                println!("Failed to remove file.");
            }
        }

        FileOperation::Pwd => {
            let status = Command::new("pwd")
                .status()
                .expect("Failed to execute pwd");

            if !status.success() {
                println!("Error getting working directory");
            }
        }
    }
}

// Function to read user input
fn read_input(prompt: &str) -> String {
    let mut input = String::new();

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
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

        let choice = read_input("\nEnter your choice (0-5): ");

        match choice.as_str() {
            "1" => {
                let dir = read_input("Enter directory path: ");
                perform_operation(FileOperation::List(dir));
            }

            "2" => {
                let file = read_input("Enter file path: ");
                perform_operation(FileOperation::Display(file));
            }

            "3" => {
                let file = read_input("Enter file path: ");
                let content = read_input("Enter content: ");
                perform_operation(FileOperation::Create(file, content));
            }

            "4" => {
                let file = read_input("Enter file path: ");
                perform_operation(FileOperation::Remove(file));
            }

            "5" => {
                perform_operation(FileOperation::Pwd);
            }

            "0" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Invalid option. Please try again.");
            }
        }
    }
}
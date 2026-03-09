use std::process::Command;
use std::io::{self, Write};


enum FileOperation {
    List(String),               // Directory path
    Display(String),            // File path
    Create(String, String),     // File path and content
    Remove(String),             // File path
    Pwd,                        // Print working directory
}

fn perform_operation(operation: FileOperation) {
    match operation{
        FileOperation::List(path) => {
            println!("Listing directory: {}", path);
            let output = Command::new("ls")
                .arg(&path)
                .output()
                .expect("Failed to execute ls command");
            
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                print!("{}", stdout);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Error listing directory: {}", stderr);
            }
        }
        FileOperation::Display(path) => {
            println!("Displaying file: {}", path);
            let output = Command::new("cat")
            .arg(&path)
            .output()
            .expect("Failed to execute cat command");
            
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                print!("{}", stdout);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Error displaying file: {}", stderr);
            }
        }
        FileOperation::Create(path, content) => {
            println!("Creating file: {}", path);
            let command = format!("echo '{}' > {}", content, path);
            let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .expect("Failed to execute create command");
            
            if output.status.success() {
                println!("File '{}' created successfully.", path);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Failed to create file: {}", stderr);
            }
        }
        FileOperation::Remove(path) => {
            println!("Removing file: {}", path);
            let output = Command::new("rm")
            .arg(&path)
            .output()
            .expect("Failed to execute rm command");
            
            if output.status.success() {
                println!("File '{}' removed successfully.", path);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Failed to remove file: {}", stderr);
            }
        }
        FileOperation::Pwd => {
            println!("Current working directory:");
            let output = Command::new("pwd")
                .output()
                .expect("Failed to execute pwd command");
            
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                print!("{}", stdout);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Error getting working directory: {}", stderr);
            }
        }
    }
    // Implement command execution based on the operation
}



fn main() {
    println!("Welcome to the File Operations Program!\n");
    loop {
        println!("File Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");
        print!("Enter your choice (0-5): ");

        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim();

        match choice {
            "0" => {
                println!("Goodbye!");
                break;
            }
            "1" => {
                print!("Enter directory path: ");
                io::stdout().flush().unwrap();
                let mut path = String::new();
                io::stdin().read_line(&mut path).expect("Failed to read line");
                let path = path.trim().to_string();
                perform_operation(FileOperation::List(path));
            }
            "2" => {
                print!("Enter file path: ");
                io::stdout().flush().unwrap();
                let mut path = String::new();
                io::stdin().read_line(&mut path).expect("Failed to read line");
                let path = path.trim().to_string();
                perform_operation(FileOperation::Display(path));
            }
            "3" => {
                print!("Enter file path: ");
                io::stdout().flush().unwrap();
                let mut path = String::new();
                io::stdin().read_line(&mut path).expect("Failed to read line");
                let path = path.trim().to_string();
                print!("Enter content: ");
                io::stdout().flush().unwrap();
                let mut content = String::new();
                io::stdin().read_line(&mut content).expect("Failed to read line");
                let content = content.trim_end().to_string();
                perform_operation(FileOperation::Create(path, content));
            }
            "4" => {
                print!("Enter file path: ");
                io::stdout().flush().unwrap();
                let mut path = String::new();
                io::stdin().read_line(&mut path).expect("Failed to read line");
                let path = path.trim().to_string();
                perform_operation(FileOperation::Remove(path));
            }
            "5" => {
                perform_operation(FileOperation::Pwd);
            }
            _ => {
                println!("Invalid choice. Please enter a number between 0 and 5.");
            }
        }
        println!();
    }
}

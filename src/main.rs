use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug)]
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            completed: false,
        }
    }

    fn mark_completed(&mut self) {
        self.completed = true;
    }

    fn display(&self) {
        let status = if self.completed { "✓" } else { "✗" };
        println!("[{}] {}", status, self.description);
    }
}

fn main() {
    let mut tasks: Vec<Task> = vec![];

    loop {
        println!("\nTo-Do List Application");
        println!("1. Add task");
        println!("2. View tasks");
        println!("3. Mark task as completed");
        println!("4. Save tasks");
        println!("5. Load tasks");
        println!("6. Exit");

        let choice = get_input("Enter your choice: ");
        match choice.trim() {
            "1" => {
                let description = get_input("Enter task description: ");
                tasks.push(Task::new(description));
            }
            "2" => {
                for (i, task) in tasks.iter().enumerate() {
                    print!("{}: ", i + 1);
                    task.display();
                }
            }
            "3" => {
                let index = get_input("Enter task number to mark as completed: ")
                    .trim()
                    .parse::<usize>()
                    .expect("Please enter a valid number");
                if index > 0 && index <= tasks.len() {
                    tasks[index - 1].mark_completed();
                } else {
                    println!("Invalid task number");
                }
            }
            "4" => {
                save_tasks(&tasks, "tasks.txt").expect("Failed to save tasks");
            }
            "5" => {
                tasks = load_tasks("tasks.txt").expect("Failed to load tasks");
            }
            "6" => break,
            _ => println!("Invalid choice"),
        }
    }
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn save_tasks(tasks: &[Task], filename: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).create(true).open(filename)?;
    for task in tasks {
        writeln!(file, "{}|{}", task.description, task.completed)?;
    }
    Ok(())
}

fn load_tasks(filename: &str) -> io::Result<Vec<Task>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut tasks = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() == 2 {
            let description = parts[0].to_string();
            let completed = parts[1] == "true";
            tasks.push(Task { description, completed });
        }
    }
    Ok(tasks)
}

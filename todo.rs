#!/usr/bin/env cargo script
//! file: todo.rs
//! description: Professional Single‐File ToDo CLI in Rust
//! author: bocaletto-luca
//! license: MIT
//!
//! A simple, cross-platform task manager in one file.
//! Stores tasks in JSON under your OS’s data directory.
//!
//! # Usage
//! ```bash
//! # Add a new task
//! todo add "Buy groceries"
//!
//! # List all tasks
//! todo list
//!
//! # Mark a task done
//! todo done 3
//!
//! # Remove a task
//! todo remove 2
//!
//! # Clear all tasks
//! todo clear
//! ```
//!
//! # Build & Run
//! ```bash
//! # Make sure you have Rust and cargo-script installed:
//! cargo install cargo-script
//! # Run directly:
//! ./todo.rs add "Write README"
//! ```
//!
//! # Dependencies (auto-installed by cargo script)
//! clap = { version = "4.3", features = ["derive"] }
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! directories = "4.4"

use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{self, BufReader, Write},
    path::PathBuf,
    process::exit,
};

/// CLI definition
#[derive(Parser)]
#[command(name = "todo", about = "Simple ToDo CLI in Rust")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add { description: Vec<String> },
    /// List all tasks
    List,
    /// Mark a task done
    Done { id: usize },
    /// Remove a task
    Remove { id: usize },
    /// Clear all tasks
    Clear,
}

/// A single task record
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    done: bool,
}

/// Return path to tasks.json in OS‐specific data directory
fn data_file() -> io::Result<PathBuf> {
    let proj = ProjectDirs::from("com", "bocaletto-luca", "todo")
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Cannot determine data directory"))?;
    let dir = proj.data_dir();
    fs::create_dir_all(dir)?;
    Ok(dir.join("tasks.json"))
}

/// Load task list or return empty
fn load_tasks() -> io::Result<Vec<Task>> {
    let path = data_file()?;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader)?;
    Ok(tasks)
}

/// Save task list to JSON
fn save_tasks(tasks: &[Task]) -> io::Result<()> {
    let path = data_file()?;
    let mut file = File::create(path)?;
    let data = serde_json::to_string_pretty(tasks)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

/// Generate next unique task ID
fn next_id(tasks: &[Task]) -> usize {
    tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

fn main() {
    let cli = Cli::parse();
    let mut tasks = match load_tasks() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error loading tasks: {}", e);
            exit(1);
        }
    };

    let result = match cli.cmd {
        Commands::Add { description } => {
            let desc = description.join(" ");
            let id = next_id(&tasks);
            tasks.push(Task { id, description: desc.clone(), done: false });
            save_tasks(&tasks)
                .map_err(|e| format!("Save error: {}", e))
                .map(|_| println!("[+] Added #{}: {}", id, desc))
        }
        Commands::List => {
            if tasks.is_empty() {
                println!("No tasks found.");
                Ok(())
            } else {
                for t in &tasks {
                    let mark = if t.done { "[x]" } else { "[ ]" };
                    println!("{} {}: {}", mark, t.id, t.description);
                }
                Ok(())
            }
        }
        Commands::Done { id } => {
            if let Some(t) = tasks.iter_mut().find(|t| t.id == id) {
                t.done = true;
                save_tasks(&tasks)
                    .map_err(|e| format!("Save error: {}", e))
                    .map(|_| println!("[✓] Marked #{} done.", id))
            } else {
                Err(format!("Task #{} not found.", id))
            }
        }
        Commands::Remove { id } => {
            let original = tasks.len();
            tasks.retain(|t| t.id != id);
            if tasks.len() < original {
                save_tasks(&tasks)
                    .map_err(|e| format!("Save error: {}", e))
                    .map(|_| println!("[-] Removed #{}.", id))
            } else {
                Err(format!("Task #{} not found.", id))
            }
        }
        Commands::Clear => {
            tasks.clear();
            save_tasks(&tasks)
                .map_err(|e| format!("Save error: {}", e))
                .map(|_| println!("[!] All tasks cleared."))
        }
    };

    if let Err(msg) = result {
        eprintln!("Error: {}", msg);
        exit(1);
    }
}

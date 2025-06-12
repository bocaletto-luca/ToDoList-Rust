# Rust ToDo CLI 📝

A professional, single‐file command‐line task manager written in Rust.  
Stores tasks as JSON under your OS’s data directory (`$XDG_DATA_HOME` or platform equivalent).

**File:** `todo.rs`  
**Author:** [bocaletto-luca](https://github.com/bocaletto-luca)  
**License:** MIT

---

## 🚀 Features

- Add, list, mark done, remove and clear all tasks  
- Uses `clap` for a modern CLI interface  
- Persists tasks in a JSON file under your data folder  
- Cross-platform: works on Linux, macOS & Windows  
- Single‐file solution with `cargo-script`  

---

## 📋 Prerequisites

- Rust toolchain (rustc + cargo)  
- `cargo-script` for zero-boilerplate scripting

Install `cargo-script`:

## bash
    cargo install cargo-script

⚙️ Build & Run

You can run the script directly with cargo-script:
  # Run any command directly:
    cargo script todo.rs list

# Compile into `todo` executable
rustc todo.rs --extern clap --extern serde --extern serde_json --extern directories
# (Alternatively use cargo-script build:)
cargo script --build todo.rs

📖 Usage

#### Add a new task
    todo add "Buy groceries"

#### List all tasks
    todo list

#### Mark task #3 done
todo done 3

#### Remove task #2
todo remove 2

#### Clear all tasks
todo clear

## Sample Session

    $ todo list
    No tasks found.

    $ todo add "Write README"
    [+] Added #1: Write README

    $ todo add "Test CLI"
    [+] Added #2: Test CLI

    $ todo list
    [ ] 1: Write README
    [ ] 2: Test CLI

    $ todo done 1
    [✓] Marked #1 done.

    $ todo list
    [x] 1: Write README
    [ ] 2: Test CLI

    $ todo remove 2
    [-] Removed #2.

    $ todo clear
    [!] All tasks cleared.

🗂️ Data Storage

Tasks are stored in:

    Linux/macOS: ~/.local/share/todo/tasks.json

    Windows: %APPDATA%\todo\tasks.json

🤝 Contributing

    Fork this repo

    Make your changes in todo.rs

    Ensure tasks persist correctly and CLI flags work

    Submit a Pull Request

---

📄 License

This project is licensed under the MIT License © 2025 

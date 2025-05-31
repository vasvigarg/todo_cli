# Rust ToDo CLI

A simple command-line ToDo app written in Rust.  
Supports adding tasks with due dates in IST timezone, listing, marking done, deleting tasks, and persistent storage.

---

## Features

- **Add tasks** with optional due dates in IST timezone
- **List tasks** with status and due date display
- **Mark tasks as done** by index
- **Delete tasks** by index
- **Persistent storage** of tasks in a JSON file (`tasks.json`)
- **Clean CLI interface** with subcommands (using `clap`)

---

## Concepts Used

| Concept      | How We'll Use It                                     |
| ------------ | ---------------------------------------------------- |
| Ownership    | Ownership and borrowing rules for managing task data |
| Enums        | Representing task status (e.g., `TaskStatus`)        |
| Traits       | Defining common behavior for task manager            |
| Rc / RefCell | Shared, mutable list of tasks inside the app         |

---

## How to run

### Add a task with due date in IST

Use the format `YYYY-MM-DD HH:MM` in your local time; it will be converted to IST internally.

```sh
cargo run -- add "Read ZK paper" --due "2025-06-05"
```

### List all tasks

```sh
cargo run -- list
```

### Mark a task done by index

```sh
cargo run -- done 0
```

### Delete a task by index

```sh
cargo run -- delete 0
```

Tasks are saved in `tasks.json` and loaded on every run to keep your data persistent.

---

## Requirements

- Rust 1.65+ (for latest chrono and clap support)
- Internet access to download dependencies on first build

---

## Build release binary

```sh
cargo build --release
```

Copy the compiled binary from `target/release/todo` to your system path to run it anywhere.

---

Feel free to contribute or open issues for any bugs or feature requests!

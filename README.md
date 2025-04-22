# Rust CLI Task Manager

A simple command-line application written in Rust to manage your daily tasks with support for colors, filtering, and easy storage in JSON format.



## 📦 Features

- Add, list, complete, and remove tasks from the terminal.
- Filter tasks by due date, completion status, and priority.
- Colorized terminal output using environment variables.
- Data stored in a JSON file for easy portability and editing.



## 🚀 Getting Started

### 🛠 Requirements

- [Rust toolchain](https://www.rust-lang.org/tools/install)  
  To build and run this app, you'll need the Rust toolchain installed.  
  Visit the official Rust website for detailed installation instructions tailored to your operating system.


Then restart your terminal and check:

```bash
rustc --version
cargo --version
```



## 🔧 Build

Clone the repository:

```bash
git clone https://github.com/PalmoSweet/rust-taskmanager.git
cd rust-taskmanager
```

Build the release version:

```bash
cargo build --release
```

The binary will be in:

```bash
target/release/task
```

## 🏃 Running

### 📦 Development

To run the app during development:

```bash
cargo run -- <subcommand> [flags]
```

Example:

```bash
cargo run -- list --today
```

### 🧪 Running globally in terminal (MAC)

To install globally:

```bash
cp target/release/task ~/.local/bin/
```

Make sure `~/.local/bin` is in your `PATH`:

Add to `~/.bashrc`, `~/.zshrc`, etc:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Now you can run:

```bash
task list --today
```

## 🎨 Configuration (.env)

To enable colored output, create a `.task.env` file in the **home directory of your computer** make sure you name it `.task.env`

Example `.env`:

```env
TEXT_COLOR=green
BACKGROUND_COLOR=none
HEADER_COLOR=white
HEADER_BACKGROUND_COLOR=green
```

These colors are used to style the task list output.

### Supported Colors

- `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`
- `none` (to disable background)
- All colors supported by the [colored](https://crates.io/crates/colored) crate can be used. Visit their [crate.io](https://crates.io) page for more info.



## 📁 Data

Tasks are stored in a `.tasks.json` file in your home directory. You can edit this manually if needed.


## 📄 Subcommands & Flags

Run `cargo run -- help` or `task help` (depending on your installation method) to view available subcommands and flags.


## 📬 License

MIT License — free to use, modify, and distribute.
- The Dependency `colored` is licensed under the Mozilla Public License 2.0. See [crates.io](https://crates.io/crates/colored) for more information.

## ✨ Contributions

Contributions, bug reports, and feature requests are welcome!


# 🦀 Rust Capstone Project - Beginner's Toolkit

**A comprehensive beginner-friendly guide to getting started with Rust programming**

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

---

## 📋 Project Overview

This repository contains a complete beginner's toolkit for learning Rust programming language. Created as part of the Moringa School Capstone Project, it includes working code examples, comprehensive documentation, and a step-by-step guide to help anyone get started with Rust.

###  What You'll Learn

- How to install and set up Rust on your system
- Writing your first Rust program (Hello World)
- Creating interactive programs with user input
- Using Cargo (Rust's build system and package manager)
- Understanding basic Rust syntax and concepts
- Troubleshooting common errors

---

## 📁 Repository Structure

```
rust-capstone-project/
├── README.md                      # This file
├── TOOLKIT_DOCUMENTATION.md       # Complete learning guide (READ THIS FIRST!)
├── Cargo.toml                     # Project configuration
├── src/
│   └── main.rs                   # Main Cargo project entry point
├── hello_world.rs                # Simple Hello World example
├── calculator.rs                 # Interactive calculator program
└── .gitignore                    # Git ignore file
```

---

##  Quick Start

### Prerequisites

Before you begin, make sure you have:
- A computer running Linux, macOS, or Windows
- Internet connection (for installation)
- A terminal/command prompt
- A text editor or IDE (VS Code recommended)

### Installation

1. **Clone this repository:**
   ```bash
   git clone https://github.com/yourusername/rust-capstone-project.git
   cd rust-capstone-project
   ```

2. **Install Rust** (if you haven't already):
   
   **Linux/macOS:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```
   
   **Windows:**
   - Download from: https://rustup.rs/
   - Run the installer and follow instructions

3. **Verify installation:**
   ```bash
   rustc --version
   cargo --version
   ```

---

## 💻 Running the Examples

### Option 1: Using Cargo (Recommended)

```bash
# Run the main project
cargo run

# Build the project
cargo build

# Build with optimizations
cargo build --release
```

### Option 2: Direct Compilation

**Hello World:**
```bash
rustc hello_world.rs
./hello_world
```

**Calculator:**
```bash
rustc calculator.rs
./calculator
```

---

## 📖 Documentation

For the complete learning guide, including:
- Detailed installation instructions
- Step-by-step tutorials
- AI prompts used in this project
- Common issues and solutions
- Additional resources

**👉 Read the [TOOLKIT_DOCUMENTATION.md](TOOLKIT_DOCUMENTATION.md) file**

---

##  Learning Path

1. **Start here:** Read `TOOLKIT_DOCUMENTATION.md` sections 1-4 for setup
2. **First program:** Run `hello_world.rs`
3. **Interactive program:** Try `calculator.rs`
4. **Cargo project:** Explore the `src/` directory
5. **Next steps:** Check section 9 in the documentation for project ideas

---

## 🛠️ Examples Included

### 1. Hello World (`hello_world.rs`)
A simple program demonstrating basic Rust syntax:
- Printing to console
- Variables and string interpolation
- Comments

**Run it:**
```bash
rustc hello_world.rs && ./hello_world
```

**Expected Output:**
```
Hello, World!
Welcome, Rust Developer!
Learning Rust in 2024!
```

---

### 2. Simple Calculator (`calculator.rs`)
An interactive calculator demonstrating:
- Functions
- User input handling
- Pattern matching
- Error handling
- Conditional logic

**Run it:**
```bash
rustc calculator.rs && ./calculator
```

**Sample Usage:**
```
=== Simple Rust Calculator ===
Choose an operation:
1. Add
2. Subtract
3. Multiply
4. Divide
1
Enter first number:
10
Enter second number:
5
10 + 5 = 15

Result: 15
```

---

### 3. Cargo Project (`src/main.rs`)
A properly structured Rust project using Cargo:
- Standard project layout
- Dependency management with `Cargo.toml`
- Build and run with `cargo run`

**Run it:**
```bash
cargo run
```

---

## 🔧 Recommended VS Code Extensions

To enhance your Rust development experience:

1. **rust-analyzer** - Essential IDE features
   ```
   code --install-extension rust-lang.rust-analyzer
   ```

2. **CodeLLDB** - For debugging
   ```
   code --install-extension vadimcn.vscode-lldb
   ```

3. **crates** - Manage dependencies
   ```
   code --install-extension serayuzgur.crates
   ```

4. **Even Better TOML** - For Cargo.toml files
   ```
   code --install-extension tamasfe.even-better-toml
   ```

---

##  Troubleshooting

### Common Issues

**1. `rustc: command not found`**
```bash
source $HOME/.cargo/env
# Or restart your terminal
```

**2. Linker errors on Linux**
```bash
sudo apt install build-essential
```

**3. VS Code rust-analyzer not working**
- Wait for initial indexing (1-2 minutes)
- Reload window: Ctrl+Shift+P → "Reload Window"
- Make sure you opened the folder containing Cargo.toml

For more solutions, see the [Common Issues section](TOOLKIT_DOCUMENTATION.md#7-common-issues--fixes) in the documentation.

---

## 📚 Additional Resources

### Official Resources
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Cargo Book](https://doc.rust-lang.org/cargo/)
- [Rust Playground](https://play.rust-lang.org/) - Try Rust online

### Community
- [Rust Users Forum](https://users.rust-lang.org/)
- [r/rust on Reddit](https://www.reddit.com/r/rust/)
- [Rust Discord](https://discord.gg/rust-lang)

### Learning Platforms
- [Exercism - Rust Track](https://exercism.org/tracks/rust)
- [Tour of Rust](https://tourofrust.com/)
- [Rustlings](https://github.com/rust-lang/rustlings)

---

## 🤝 Contributing

This is a learning project, but suggestions and improvements are welcome!

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/improvement`)
3. Commit your changes (`git commit -am 'Add some improvement'`)
4. Push to the branch (`git push origin feature/improvement`)
5. Open a Pull Request

---

## 📝 Project Details

**Technology:** Rust Programming Language  
**Created:** December 2024  
**Purpose:** Moringa School Capstone Project  




---

##  Next Steps After This Project

Once you're comfortable with the basics:

1. **Build a CLI todo app** - Practice file I/O and data structures
2. **Create a web scraper** - Learn about HTTP requests and parsing
3. **Make a simple REST API** - Explore web frameworks like Actix or Rocket
4. **Build a game** - Try game development with Bevy
5. **Contribute to open source** - Find Rust projects on GitHub

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- **Moringa School** for the capstone project framework
- **The Rust Community** for excellent documentation and support
- **Claude AI** (ai.moringaschool.com) for learning assistance
- **rust-lang.org** for creating an amazing language

---

## 📧 Contact

**Author:** Mutsumi Nathan Ngala  
**Email:** nathanngala8@gmail.com  
**GitHub:** https://github.com/MutsumiNgala  
**Project Link:** https://github.com/MutsumiNgala/RUST-CAPSTONE-PROJECT

---


If this toolkit helped you learn Rust, please:
- ⭐ Star this repository
- 🍴 Fork it for your own learning
- 📢 Share it with others learning Rust
- 💬 Leave feedback or suggestions

---

**Happy Rust Programming! 🦀**



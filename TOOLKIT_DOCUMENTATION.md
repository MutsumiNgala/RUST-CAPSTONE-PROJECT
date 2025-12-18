# Getting Started with Rust Programming – A Beginner's Guide

**Author:** Mutsumi Nathan Ngala   
**Date:** December 18, 2025  
**Technology:** Rust Programming Language

---

## 1. Title & Objective

### What technology did i choose?
**Rust** - A modern systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.

### Why did i choose it?
I chose Rust because:
- It's gaining massive popularity in systems programming (used by Microsoft, Amazon, Google)
- It offers memory safety without garbage collection
- It has excellent documentation and a welcoming community
- It's different from Python, Java, and JavaScript (as required)
- It's the "most loved" programming language for 8 years running (Stack Overflow Developer Survey)

### What's the end goal?
By the end of this guide, you'll be able to:
1. Install Rust on your system
2. Write and run a "Hello World" program
3. Create a simple calculator application with user input
4. Understand basic Rust syntax and concepts
5. Use Cargo (Rust's package manager and build system)

---

## 2. Quick Summary of the Technology

### What is Rust?
Rust is a multi-paradigm, general-purpose programming language that emphasizes performance, type safety, and concurrency. It was created by Mozilla Research and first released in 2010. Rust enforces memory safety without using garbage collection, making it ideal for systems programming.

### Where is it used?
Rust is used in:
- **Operating Systems**: Parts of Windows, Linux kernel modules
- **Web Browsers**: Firefox, Chrome components
- **Game Engines**: Bevy, Amethyst
- **Cloud Infrastructure**: AWS, Cloudflare, Dropbox
- **Blockchain**: Solana, Polkadot
- **Command-line Tools**: ripgrep, bat, exa
- **Embedded Systems**: IoT devices, microcontrollers

### Real-World Example
**Dropbox** rewrote critical file-sync components in Rust, improving performance and reducing memory usage by over 50% while eliminating entire classes of bugs related to memory safety.

---

## 3. System Requirements

### Operating System
- ✅ **Linux** (Ubuntu, Debian, Fedora, etc.)
- ✅ **macOS** (10.12 Sierra or later)
- ✅ **Windows** (10 or later)

### Tools/Editors Required

#### Essential:
- **Rust** (rustc compiler + cargo build tool) - We'll install this!
- **Text Editor** or **IDE**:
  - **VS Code** (Recommended) - [Download here](https://code.visualstudio.com/)
  - Sublime Text
  - Vim/Neovim
  - IntelliJ IDEA with Rust plugin

#### Recommended VS Code Extensions:
1. **rust-analyzer** (by rust-lang) - Provides IDE features like autocomplete, go-to-definition
2. **CodeLLDB** (by Vadim Chugunov) - For debugging Rust code
3. **crates** (by Seray Uzgur) - Helps manage dependencies
4. **Even Better TOML** - For editing Cargo.toml files

### Additional Requirements
- **Internet connection** (for initial installation)
- **Command line/Terminal** access
- **~1.5 GB** of disk space for Rust toolchain

---

## 4. Installation & Setup Instructions

### Step 1: Install Rust

#### On Linux or macOS:
Open your terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

When prompted, press **1** to proceed with the default installation.

#### On Windows:
1. Download the installer from: https://rustup.rs/
2. Run `rustup-init.exe`
3. Follow the on-screen instructions
4. You may need to install Visual Studio C++ Build Tools if prompted

### Step 2: Configure Your Environment

After installation, configure your current shell:

```bash
source $HOME/.cargo/env
```

For future sessions, this will be automatic (added to your shell profile).

### Step 3: Verify Installation

Check that Rust is installed correctly:

```bash
rustc --version
cargo --version
```

**Expected output** (versions may vary):
```
rustc 1.75.0 (82e1608df 2023-12-21)
cargo 1.75.0 (1d8b05cdd 2023-11-20)
```

### Step 4: Update Rust (Optional but Recommended)

Keep Rust up to date:

```bash
rustup update
```

### Step 5: Set Up VS Code

1. **Install VS Code**: Download from [code.visualstudio.com](https://code.visualstudio.com/)
2. **Install rust-analyzer extension**:
   - Open VS Code
   - Go to Extensions (Ctrl+Shift+X or Cmd+Shift+X)
   - Search for "rust-analyzer"
   - Click Install

3. **Configure rust-analyzer** (Optional):
   - Go to Settings (Ctrl+,)
   - Search for "rust-analyzer"
   - Enable "Check On Save"

---

## 5. Minimal Working Examples

### Example 1: Hello World (Classic)

#### What it does:
This is the simplest Rust program. It prints "Hello, World!" to the console.

#### Code: `hello_world.rs`

```rust
// Hello World in Rust
// This is a simple program that demonstrates the basic syntax of Rust

fn main() {
    // Print to console with automatic newline
    println!("Hello, World!");
    
    // Print with variable interpolation
    let name = "Rust Developer";
    println!("Welcome, {}!", name);
    
    // Print with multiple variables
    let language = "Rust";
    let year = 2024;
    println!("Learning {} in {}!", language, year);
}
```

#### How to run:

**Method 1: Direct compilation**
```bash
rustc hello_world.rs
./hello_world
```

**Expected Output:**
```
Hello, World!
Welcome, Rust Developer!
Learning Rust in 2024!
```

---

### Example 2: Simple Calculator (Interactive)

#### What it does:
An interactive command-line calculator that performs basic arithmetic operations (addition, subtraction, multiplication, division). It demonstrates:
- Functions
- User input handling
- Pattern matching
- Error handling
- Conditional logic

#### Code: `calculator.rs`

```rust
// Simple Calculator in Rust
// This demonstrates functions, pattern matching, and user input

use std::io;

// Function to add two numbers
fn add(a: f64, b: f64) -> f64 {
    a + b
}

// Function to subtract two numbers
fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

// Function to multiply two numbers
fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

// Function to divide two numbers
fn divide(a: f64, b: f64) -> f64 {
    if b != 0.0 {
        a / b
    } else {
        println!("Error: Cannot divide by zero!");
        0.0
    }
}

fn main() {
    println!("=== Simple Rust Calculator ===");
    println!("Choose an operation:");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");
    
    // Read operation choice
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    
    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input!");
            return;
        }
    };
    
    // Read first number
    println!("Enter first number:");
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");
    let num1: f64 = num1.trim().parse().expect("Please enter a valid number");
    
    // Read second number
    println!("Enter second number:");
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");
    let num2: f64 = num2.trim().parse().expect("Please enter a valid number");
    
    // Perform calculation based on choice
    let result = match choice {
        1 => {
            println!("{} + {} = {}", num1, num2, add(num1, num2));
            add(num1, num2)
        }
        2 => {
            println!("{} - {} = {}", num1, num2, subtract(num1, num2));
            subtract(num1, num2)
        }
        3 => {
            println!("{} × {} = {}", num1, num2, multiply(num1, num2));
            multiply(num1, num2)
        }
        4 => {
            println!("{} ÷ {} = {}", num1, num2, divide(num1, num2));
            divide(num1, num2)
        }
        _ => {
            println!("Invalid operation!");
            0.0
        }
    };
    
    println!("\nResult: {}", result);
}
```

#### How to run:

```bash
rustc calculator.rs
./calculator
```

**Sample Interaction:**
```
=== Simple Rust Calculator ===
Choose an operation:
1. Add
2. Subtract
3. Multiply
4. Divide
1
Enter first number:
15
Enter second number:
25
15 + 25 = 40

Result: 40
```

---

### Example 3: Using Cargo (Recommended Workflow)

Cargo is Rust's build system and package manager. It's the standard way to manage Rust projects.

#### Create a new project:

```bash
cargo new my_rust_app
cd my_rust_app
```

This creates:
```
my_rust_app/
├── Cargo.toml       # Project metadata and dependencies
└── src/
    └── main.rs      # Main source file
```

#### Edit `src/main.rs`:

```rust
fn main() {
    println!("🦀 Welcome to Rust Programming!");
    println!("================================");
    println!();
    println!("This is your first Rust program!");
    println!();
    println!("Rust is a systems programming language that focuses on:");
    println!("  ✓ Memory safety");
    println!("  ✓ Speed");
    println!("  ✓ Concurrency");
    println!();
    println!("Happy coding! 🚀");
}
```

#### Run with Cargo:

```bash
cargo run
```

**Output:**
```
   Compiling my_rust_app v0.1.0 (/path/to/my_rust_app)
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/my_rust_app`
🦀 Welcome to Rust Programming!
================================

This is your first Rust program!

Rust is a systems programming language that focuses on:
  ✓ Memory safety
  ✓ Speed
  ✓ Concurrency

Happy coding! 🚀
```

#### Other useful Cargo commands:

```bash
cargo build        # Compile the project
cargo build --release  # Compile with optimizations
cargo check        # Check if code compiles (faster than build)
cargo test         # Run tests
cargo doc --open   # Generate and open documentation
```

---

## 6. AI Prompt Journal

This section documents all AI prompts used during the learning process and their effectiveness.

### Prompt 1: Understanding Rust Basics

**Prompt used:**  
*"Explain what Rust programming language is, its main features, and why developers choose it over other languages like C++ or Java. Keep it beginner-friendly."*

**AI Response Summary:**  
The AI explained that Rust is a systems programming language focused on safety, speed, and concurrency. It highlighted memory safety without garbage collection, zero-cost abstractions, and fearless concurrency as key features. The response also mentioned Rust's growing adoption in industry.


**Evaluation:**  
⭐⭐⭐⭐⭐ (5/5) - Excellent. The explanation was clear and helped me understand Rust's unique value proposition compared to other languages I already know.

---

### Prompt 2: Installation Guidance

**Prompt used:**  
*"Give me step-by-step instructions to install Rust on Ubuntu Linux, including how to verify the installation was successful."*

**AI Response Summary:**  
The AI provided the rustup installation command, explained what rustup is (Rust's toolchain installer), and gave verification commands. It also mentioned how to update Rust in the future.


**Evaluation:**  
⭐⭐⭐⭐⭐ (5/5) - Perfect. The instructions were precise and worked exactly as described. The AI also mentioned potential issues with PATH configuration.

---

### Prompt 3: Hello World Implementation

**Prompt used:**  
*"Write a simple 'Hello World' program in Rust with detailed comments explaining what each part does. Also explain how to compile and run it."*

**AI Response Summary:**  
The AI provided a clean Hello World example with comments explaining `fn main()`, `println!` macro, and string interpolation. It showed both direct compilation with `rustc` and the Cargo workflow.


**Evaluation:**  
⭐⭐⭐⭐☆ (4/5) - Very good. The code worked perfectly, but I had to ask a follow-up question about the difference between `rustc` and `cargo`.

---

### Prompt 4: Understanding Cargo

**Prompt used:**  
*"What is Cargo in Rust? How do I use it to create and manage a Rust project? Explain like I'm coming from Python's pip or Node's npm."*

**AI Response Summary:**  
The AI explained Cargo as Rust's "package manager + build tool + test runner" all in one. It compared it to pip+virtualenv for Python or npm+webpack for Node.js. Provided commands for creating projects, adding dependencies, and building.


**Evaluation:**  
⭐⭐⭐⭐⭐ (5/5) - Excellent analogy to tools I already know. The comparison to npm/pip made it immediately clear.

---

### Prompt 5: Building an Interactive Program

**Prompt used:**  
*"Help me create a simple calculator in Rust that takes user input and performs basic arithmetic operations. Include error handling for invalid inputs and division by zero."*

**AI Response Summary:**  
The AI provided a complete calculator implementation with separate functions for each operation, pattern matching for the menu, and error handling using `match` and `expect()`. It explained the `std::io` module for user input.


**Evaluation:**  
⭐⭐⭐⭐⭐ (5/5) - Outstanding. This really helped me understand Rust's approach to error handling and pattern matching. The code was well-structured and educational.

---

### Prompt 6: Debugging Compilation Errors

**Prompt used:**  
*"I'm getting an error 'cannot borrow `choice` as mutable more than once'. What does this mean in Rust and how do I fix it?"*

**AI Response Summary:**  
The AI explained Rust's borrowing rules and ownership system. It clarified that the error occurs when trying to use a variable after it's been moved or borrowed. Provided the fix: limiting the scope of borrows and understanding move semantics.


**Evaluation:**  
⭐⭐⭐⭐☆ (4/5) - Very helpful. Ownership is a tricky concept, and the AI's explanation made it clearer, though I needed to read it twice to fully understand.

---

### Prompt 7: VS Code Setup

**Prompt used:**  
*"What VS Code extensions should I install for Rust development? How do I configure them for the best experience?"*

**AI Response Summary:**  
The AI recommended rust-analyzer as the essential extension, along with CodeLLDB for debugging. Explained how to enable "check on save" and other useful settings. Mentioned that rust-analyzer replaced the older RLS extension.


**Evaluation:**  
⭐⭐⭐⭐⭐ (5/5) - Perfect recommendations. rust-analyzer's autocomplete and inline error detection significantly improved my development experience.

---

### Prompt 8: Understanding Rust Syntax

**Prompt used:**  
*"Explain the difference between `let`, `let mut`, and `const` in Rust. When should I use each one?"*

**AI Response Summary:**  
The AI explained that `let` creates immutable variables (default in Rust), `let mut` creates mutable variables, and `const` creates compile-time constants. Emphasized Rust's "immutable by default" philosophy for safety.


**Evaluation:**  
⭐⭐⭐⭐⭐ (5/5) - Clear and concise. The examples showing when to use each were particularly helpful.

---

### Overall AI Learning Effectiveness

**Summary:** Using AI prompts accelerated my Rust learning significantly. Instead of spending hours reading documentation, I could ask specific questions and get targeted answers. The AI was particularly helpful for:
- Explaining concepts in relation to languages I already know
- Providing working code examples
- Debugging compilation errors
- Setting up the development environment

**Areas where AI needed improvement:**
- Sometimes assumed more background knowledge than I had
- Occasionally provided outdated information (e.g., mentioning RLS instead of rust-analyzer initially)

**Time saved:** Estimated 3-4 hours compared to learning entirely from documentation.

---

## 7. Common Issues & Fixes

### Issue 1: `rustc: command not found`

**Problem:**  
After installing Rust, the terminal doesn't recognize `rustc` or `cargo` commands.

**Cause:**  
The Rust binaries are not in your system's PATH.

**Solution:**

```bash
# Run this in your current terminal session
source $HOME/.cargo/env

# Or restart your terminal
# The installation script should have added this to your shell profile automatically
```

**Verify fix:**
```bash
which rustc
# Should output: /home/yourusername/.cargo/bin/rustc
```

---

### Issue 2: `linker 'cc' not found` (Linux)

**Problem:**  
Error during compilation: `error: linker 'cc' not found`

**Cause:**  
Missing C compiler (Rust uses the system's C linker).

**Solution:**

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install build-essential
```

**Fedora:**
```bash
sudo dnf install gcc
```

**Arch Linux:**
```bash
sudo pacman -S base-devel
```

---

### Issue 3: `cannot borrow as mutable more than once`

**Problem:**  
Compilation error related to borrowing rules.

**Example:**
```rust
let mut x = String::from("hello");
let r1 = &mut x;
let r2 = &mut x;  // ERROR!
println!("{}, {}", r1, r2);
```

**Cause:**  
Rust's ownership rules prevent multiple mutable references to the same data.

**Solution:**  
Limit the scope of borrows:

```rust
let mut x = String::from("hello");
{
    let r1 = &mut x;
    // use r1
} // r1 goes out of scope here

let r2 = &mut x;  // OK!
```

**Learning resource:**  
[The Rust Book - References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

---

### Issue 4: `thread 'main' panicked at 'assertion failed'`

**Problem:**  
Program crashes with panic message.

**Cause:**  
Using `.expect()` or `.unwrap()` on an error result, or an assertion failed.

**Solution:**  
Use proper error handling with `match`:

**Instead of:**
```rust
let num: i32 = input.trim().parse().expect("Not a number!");
```

**Use:**
```rust
let num: i32 = match input.trim().parse() {
    Ok(n) => n,
    Err(_) => {
        println!("Invalid input!");
        return;
    }
};
```

---

### Issue 5: VS Code rust-analyzer not working

**Problem:**  
rust-analyzer extension shows errors or doesn't provide autocomplete.

**Possible causes & solutions:**

1. **Rust not installed properly:**
   ```bash
   rustup update
   rustc --version
   ```

2. **Extension not fully loaded:**
   - Look for "rust-analyzer" in bottom status bar
   - Wait for it to finish indexing (first time can take 1-2 minutes)

3. **Wrong workspace folder:**
   - Open the folder containing `Cargo.toml`, not a parent folder

4. **Restart extension:**
   - Press Ctrl+Shift+P (Cmd+Shift+P on Mac)
   - Type "Reload Window"
   - Press Enter

---

### Issue 6: Slow compile times

**Problem:**  
Rust compilation seems slow, especially for small changes.

**Solutions:**

1. **Use `cargo check` instead of `cargo build`** for quick syntax checking:
   ```bash
   cargo check  # Much faster, doesn't produce executable
   ```

2. **Use incremental compilation** (enabled by default in new Rust versions):
   ```bash
   cargo build  # Subsequent builds are faster
   ```

3. **Install `sccache`** for caching compiled dependencies:
   ```bash
   cargo install sccache
   export RUSTC_WRAPPER=sccache
   ```

4. **Use `cargo watch` for automatic recompilation:**
   ```bash
   cargo install cargo-watch
   cargo watch -x check
   ```

---

### Issue 7: Windows-specific: Missing Visual Studio Build Tools

**Problem:**  
Error: `error: linker 'link.exe' not found`

**Cause:**  
Windows requires Microsoft Visual C++ build tools.

**Solution:**

1. Download Visual Studio Installer: https://visualstudio.microsoft.com/downloads/
2. Choose "Build Tools for Visual Studio"
3. Select "C++ build tools" workload
4. Install
5. Restart terminal and try again

**Alternative:**  
Use the GNU toolchain instead:
```bash
rustup default stable-gnu
```

---

### Issue 8: "Help! My program won't stop running!"

**Problem:**  
Created an infinite loop or the program is waiting for input.

**Solution:**  
Press `Ctrl+C` in the terminal to stop the program.

---

## 8. References

### Official Documentation

1. **The Rust Programming Language Book** (The Bible of Rust)  
   https://doc.rust-lang.org/book/  
   📘 Complete guide from basics to advanced topics

2. **Rust by Example**  
   https://doc.rust-lang.org/rust-by-example/  
   🎯 Learn Rust through annotated example programs

3. **The Cargo Book**  
   https://doc.rust-lang.org/cargo/  
   📦 Everything about Rust's package manager

4. **Rust Standard Library Documentation**  
   https://doc.rust-lang.org/std/  
   📚 API documentation for all built-in types and modules

5. **Rustlings** (Interactive exercises)  
   https://github.com/rust-lang/rustlings  
   🎓 Small exercises to get you used to reading and writing Rust code

---

### Video Tutorials

1. **"Rust Programming Course for Beginners"** by freeCodeCamp  
   https://www.youtube.com/watch?v=MsocPEZBd-M  
   ⏱️ Duration: ~2 hours  
   📹 Comprehensive introduction to Rust

2. **"Rust Crash Course"** by Traversy Media  
   https://www.youtube.com/watch?v=zF34dRivLOw  
   ⏱️ Duration: ~1 hour  
   📹 Quick overview of Rust basics

3. **Let's Get Rusty** (YouTube Channel)  
   https://www.youtube.com/c/LetsGetRusty  
   📹 Regular Rust tutorials and news

---

### Interactive Learning Platforms

1. **Exercism - Rust Track**  
   https://exercism.org/tracks/rust  
   🏋️ 100+ coding exercises with mentorship

2. **Tour of Rust**  
   https://tourofrust.com/  
   🚀 Interactive tour through Rust features

3. **Rust Playground**  
   https://play.rust-lang.org/  
   🎮 Write and run Rust code in your browser

---

### Community & Help

1. **Rust Users Forum**  
   https://users.rust-lang.org/  
   💬 Ask questions, share projects

2. **r/rust** (Reddit)  
   https://www.reddit.com/r/rust/  
   🗣️ Active community discussions

3. **Rust Discord**  
   https://discord.gg/rust-lang  
   💬 Real-time chat for help and discussion

4. **Stack Overflow - Rust Tag**  
   https://stackoverflow.com/questions/tagged/rust  
   ❓ Q&A for specific problems

---

### Helpful Blog Posts & Articles

1. **"Why Rust?"** by Discord  
   https://discord.com/blog/why-discord-is-switching-from-go-to-rust  
   📝 Real-world case study

2. **"Rust: A Language for the Next 40 Years"** by Carol Nichols  
   https://www.youtube.com/watch?v=A3AdN7U24iU  
   🎤 Conference talk about Rust's future

3. **"Rust for Python Programmers"**  
   https://lucumr.pocoo.org/2015/5/27/rust-for-pythonistas/  
   📝 Great if you know Python

---

### Tools & Extensions

1. **rust-analyzer** (VS Code Extension)  
   https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer  
   🔧 Essential IDE support

2. **Clippy** (Rust Linter)  
   ```bash
   rustup component add clippy
   cargo clippy
   ```
   ✅ Catches common mistakes and suggests improvements

3. **rustfmt** (Code Formatter)  
   ```bash
   rustup component add rustfmt
   cargo fmt
   ```
   🎨 Automatically format your code

---

### Books (Free Online)

1. **"The Rust Programming Language"** (Official Book)  
   https://doc.rust-lang.org/book/  
   📖 Free online, also available in print

2. **"Programming Rust"** by O'Reilly (Paid, but excellent)  
   https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/  
   📚 Deep dive into Rust

3. **"Rust for Rustaceans"** by Jon Gjengset  
   https://rust-for-rustaceans.com/  
   📕 Intermediate to advanced topics

---

### Quick Reference Cheat Sheets

1. **Rust Language Cheat Sheet**  
   https://cheats.rs/  
   📄 Quick syntax reference

2. **Rust Container Cheat Sheet**  
   https://docs.google.com/presentation/d/1q-c7UAyrUlM-eZyTo1pd8SZ0qwA_wYxmPZVOQkoDmH4/  
   📊 When to use Vec, HashMap, etc.

---

### AI Assistance (Used in This Project)

1. **Claude AI** (ai.moringaschool.com)  
   🤖 Used for learning prompts and troubleshooting

---

## 9. Next Steps & Further Learning

 You've completed the basics of Rust. Here's what to explore next:

### Beginner+ Topics:
1. **Ownership and Borrowing** (Deep dive)
2. **Error Handling** (Result and Option types)
3. **Structs and Enums**
4. **Pattern Matching**
5. **Collections** (Vec, HashMap, HashSet)

### Intermediate Topics:
1. **Traits** (Rust's interfaces)
2. **Lifetimes**
3. **Closures and Iterators**
4. **Smart Pointers**
5. **Concurrency** (Threads, async/await)

### Project Ideas:
1. Build a CLI todo app
2. Create a web scraper
3. Make a simple HTTP server
4. Build a file encryption tool
5. Create a mini game (text-based adventure)

---

## 10. Conclusion

This toolkit has introduced you to Rust programming, from installation to writing interactive programs. Rust has a steeper learning curve than some languages, but its safety guarantees and performance make it worthwhile.

**Key Takeaways:**
- ✅ Rust prevents many common bugs at compile time
- ✅ Cargo makes project management easy
- ✅ The Rust community is welcoming and helpful
- ✅ AI tools can significantly accelerate learning


---

**Project Repository:** https://github.com/MutsumiNgala/RUST-CAPSTONE-PROJECT  
**Contact:**nathanngala8@gmail.com  
**License:** MIT

🦀 Happy Rust Programming! 🦀

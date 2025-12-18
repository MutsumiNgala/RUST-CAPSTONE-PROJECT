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

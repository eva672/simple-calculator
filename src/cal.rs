// A simple CLI in rust
// This CLI will get a string in input
//and return the string

// import the env module to get the command line arguments
     
use std::io;

fn calculator(a: f64, b: f64, operator: char) -> Result<f64, String> {
    match operator {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' => {
            if b == 0.0 {
                Err(String::from("Error: Division by zero"))
            } else {
                Ok(a / b)
            }
        }
        _ => Err(String::from("Error: Unsupported operator")),
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter an operation");

    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 3 {
        println!("Error: Please enter an expression in the format: number operator number");
        return;
    }

    let a: f64 = match parts[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Invalid first number");
            return;
        }
    };

    let b: f64 = match parts[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Invalid second number");
            return;
        }
    };

    let operator = parts[1].chars().next().unwrap();

    match calculator(a, b, operator) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("{}", e),
    }
}


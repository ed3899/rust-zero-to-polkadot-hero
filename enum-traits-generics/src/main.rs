// We need the "io" (input/output) module to read from the keyboard.
use std::io;

// Define all possible operations our calculator can do.
// An enum is like a list of choices.
enum Operation {
    Add, // addition
    Sub, // subtraction
    Mul, // multiplication
    Div, // division
}

// A "trait" is like a rulebook.
// Anything that follows Compute must have an apply() function
// that takes two numbers (a and b) and gives back one number.
trait Compute {
    fn apply(&self, a: f64, b: f64) -> f64;
}

// Here we say: "Operation follows the Compute rulebook."
impl Compute for Operation {
    fn apply(&self, a: f64, b: f64) -> f64 {
        // Look at what kind of Operation we have,
        // then do the matching math.
        match self {
            Operation::Add => a + b,
            Operation::Sub => a - b,
            Operation::Mul => a * b,
            Operation::Div => a / b,
        }
    }
}

fn main() {
    // Start an infinite loop (keeps running until we break out).
    loop {
        println!("Enter operation (+, -, *, /) or q to quit:");

        // Make a new empty String to hold the user's input.
        let mut op_input = String::new();

        // Read a line of text from the keyboard into op_input.
        // unwrap() just says "if there’s an error, crash".
        io::stdin().read_line(&mut op_input).unwrap();

        // Remove spaces and newlines from the input.
        let op_input = op_input.trim();

        // If the user typed 'q', quit the program.
        if op_input == "q" {
            break;
        }

        // Convert the input symbol into one of our Operation choices.
        let op = match op_input {
            "+" => Operation::Add,
            "-" => Operation::Sub,
            "*" => Operation::Mul,
            "/" => Operation::Div,
            // If it's none of those, tell the user it's invalid and restart the loop.
            _ => {
                println!("Invalid operation");
                continue;
            }
        };

        // Ask for the first number
        let mut a = String::new();
        println!("Enter first number:");
        io::stdin().read_line(&mut a).unwrap();

        // Ask for the second number
        let mut b = String::new();
        println!("Enter second number:");
        io::stdin().read_line(&mut b).unwrap();

        // Turn the text inputs into actual numbers (f64 = decimal numbers).
        let a: f64 = a.trim().parse().unwrap();
        let b: f64 = b.trim().parse().unwrap();

        // Use the chosen operation's apply() method to calculate the result.
        println!("Result = {}", op.apply(a, b));
    }
}

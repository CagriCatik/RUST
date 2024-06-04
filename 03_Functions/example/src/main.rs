// Rust code for Functions

// Function to calculate factorial
fn factorial(n: u32) -> u32 {
    if n == 0 {
        1 // Base case: 0! = 1
    } else {
        n * factorial(n - 1) // Recursive case: n! = n * (n-1)!
    }
}

fn main() {
    let num = 5;
    let result = factorial(num);
    println!("Factorial of {} is: {}", num, result); // Output: Factorial of 5 is: 120
}

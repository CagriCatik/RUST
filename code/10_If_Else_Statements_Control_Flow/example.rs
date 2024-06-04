// Rust code for Comments

fn main() {
    // Example 1: Using a loop to find the factorial of a number
    let mut num = 5;
    let mut factorial = 1;

    println!("Calculating factorial of {}...", num);
    while num > 0 {
        factorial *= num;
        num -= 1;
    }
    println!("Factorial: {}", factorial);

    // Example 2: Using a loop to search for a specific element in an array
    let numbers = [10, 20, 30, 40, 50];
    let target = 30;
    let mut found = false;

    println!("Searching for {} in the array...", target);
    for &num in numbers.iter() {
        if num == target {
            found = true;
            break;
        }
    }
    if found {
        println!("{} found in the array!", target);
    } else {
        println!("{} not found in the array.", target);
    }

    // Example 3: Using a loop to generate a Fibonacci sequence
    let mut a = 0;
    let mut b = 1;
    let mut fib_sequence = vec![a, b];

    println!("Generating Fibonacci sequence...");
    while fib_sequence.len() < 10 {
        let next = a + b;
        fib_sequence.push(next);
        a = b;
        b = next;
    }
    println!("Fibonacci sequence: {:?}", fib_sequence);
}

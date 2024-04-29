# Functions

In this tutorial, we'll delve into functions in Rust. Up until now, we've primarily worked with the `main` function, which serves as the entry point for Rust programs. Let's explore how to define and use functions beyond the `main` function.

## 1. Understanding Functions in Rust

In Rust, any function starts with the keyword `fn`, followed by the function's name. It's essential to note that the `main` function serves as the entry point for your program, and changing its name will lead to compilation errors.

```rust
fn main() {
    // Your code here
}
```

## 2. Naming Conventions

- Functions and variables in Rust should follow the snake case convention, where words are separated by underscores (`_`).
- Avoid using kebab case for functions and variables.

Example:

```rust
fn hello_world() {
    println!("Hello, world!");
}
```

## 3. Defining Functions with Parameters

Functions can accept parameters, which are specified within the parentheses. Let's create a function `tell_height` that prints a message about the height passed to it.

```rust
fn tell_height(height: i32) {
    println!("My height is {} cm", height);
}
```

## 4. Function Invocation

Invoke a function by writing its name followed by parentheses. If the function requires arguments, provide them within the parentheses.

```rust
fn main() {
    tell_height(182);
}
```

## 5. Functions with Multiple Parameters

You can define functions with multiple parameters separated by commas within the parentheses.

```rust
fn human_id(name: &str, age: u32, height: f32) {
    println!("My name is {}, I am {} years old, and my height is {} cm", name, age, height);
}
```

## 6. Returning Values from Functions

Functions in Rust can return values using the `->` syntax followed by the return type.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## 7. Function Expression vs. Statement

- **Expression**: Anything that returns a value.
- **Statement**: Anything that does not return a value and usually ends with a semicolon (`;`).

Example of an expression:

```rust
let result = add(4, 6);
```

Example of a statement:

```rust
let x = 5;
```

## 8. Example: BMI Calculation Function

Let's create a function to calculate the Body Mass Index (BMI) using the formula: BMI = weight (kg) / (height (m) * height (m)).

```rust
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}
```

## 9. Using the BMI Function

```rust
fn main() {
    let weight = 75.0; // in kg
    let height = 1.75; // in meters

    let bmi = calculate_bmi
```

## 10. Organizing Functions in Rust

It's common practice to organize your Rust code by defining functions before their usage, although Rust allows for flexibility in function declaration order due to hoisting.

```rust
fn main() {
    say_hello();
}

fn say_hello() {
    println!("Hello, Rust!");
}
```

## 11. Function Overloading and Default Parameters

Rust doesn't support function overloading in the traditional sense, but you can achieve similar behavior using traits and generics. Additionally, Rust doesn't provide default parameter values.

## 12. Unit Tests for Functions

Rust provides built-in support for unit testing through the `#[test]` attribute. You can write test functions within your code, and Rust's testing framework will execute them when running `cargo test`.

```rust
#[test]
fn test_bmi_calculation() {
    assert_eq!(calculate_bmi(75.0, 1.75), 24.49);
}
```

## 13. Documentation for Functions

Documentation is crucial for maintaining and sharing Rust code. You can add documentation comments (doc comments) above functions using `///`.

```rust
/// Calculates the Body Mass Index (BMI) using the formula: weight (kg) / (height (m) * height (m)).
///
/// # Arguments
///
/// * `weight_kg` - The weight in kilograms.
/// * `height_m` - The height in meters.
///
/// # Returns
///
/// The calculated BMI as a floating-point number.
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}
```

## 14. Error Handling in Functions

Rust emphasizes safety and correctness, so error handling is an essential aspect of writing robust code. Functions can return `Result` or `Option` types to handle potential errors or absence of values gracefully.

```rust
fn parse_int(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}
```

## 15. Lifetimes in Functions

When working with references in Rust, understanding lifetimes is crucial. Functions with references as parameters may require explicit lifetime annotations to ensure memory safety.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

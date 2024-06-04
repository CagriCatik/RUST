# Functions

## Introduction to Functions in Rust

In this lesson, we will delve into the concept of functions in Rust. Until now, you have encountered only one function, the `main` function. This tutorial will provide a detailed understanding of defining and using functions in Rust, adhering to scientific rigor and addressing potential inaccuracies.

### The Main Function

The `main` function is the entry point of every Rust program. When you navigate to your project's folder and execute `cargo run`, the `main` function is invoked. For instance, the following simple program will print "Hello, world!" after compilation:

```rust
fn main() {
    println!("Hello, world!");
}
```

If you rename the `main` function to something else, such as `hello`, Rust will produce errors because it cannot find the `main` function:

- `function hello is never used`
- `E0601: main function not found in crate`

The `main` function must always be present as it serves as the entry point for the program.

### Defining Functions

In Rust, any function starts with the keyword `fn`, followed by the function name, parentheses, and curly braces encompassing the function body. For example:

```rust
fn my_function() {
    // function body
}
```

#### Naming Conventions

Rust uses snake_case for function and variable names. This means all letters should be lowercase and words are separated by underscores. For example:

```rust
fn hello_world() {
    println!("Hello, Rust!");
}
```

Avoid using kebab-case (e.g., `hello-world`) as it is not permitted in Rust function names.

### Function Parameters and Return Values

Functions can accept parameters and return values. Parameters are specified within the parentheses following the function name, and the return type is specified after an arrow (`->`).

#### Example: Simple Function

Here is a simple function that prints a message:

```rust
fn hello_world() {
    println!("Hello, Rust!");
}
```

To use this function, call it from the `main` function:

```rust
fn main() {
    hello_world();
}
```

#### Example: Function with Parameters

Functions can accept parameters of various types. Here’s an example function that takes an integer parameter and prints it:

```rust
fn print_height(height: i32) {
    println!("My height is {} cm.", height);
}

fn main() {
    print_height(182);
}
```

### Function Hoisting

In Rust, you can define functions either before or after the `main` function. This is known as function hoisting. For example:

```rust
fn main() {
    greet();
}

fn greet() {
    println!("Hello, Rust!");
}
```

### Expressions and Statements

Understanding expressions and statements is crucial in Rust. 

- **Expression:** Anything that returns a value.
- **Statement:** Anything that does not return a value.

For example, the following are expressions:

```rust
5
true
3 + 4
```

An `if` condition is also an expression in Rust:

```rust
let result = if condition { value1 } else { value2 };
```

### Returning Values from Functions

Functions in Rust can return values using expressions. The return type is specified after the arrow (`->`):

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add(4, 6);
    println!("Sum: {}", sum);
}
```

### Example: Calculating BMI

Let's create a function that calculates the Body Mass Index (BMI). The formula for BMI is weight in kilograms divided by the square of height in meters.

```rust
fn calculate_bmi(weight: f64, height: f64) -> f64 {
    weight / (height * height)
}

fn main() {
    let weight = 70.0;
    let height = 1.75;
    let bmi = calculate_bmi(weight, height);
    println!("Your BMI is {:.2}", bmi);
}
```

### Conclusion

This tutorial has provided a comprehensive overview of functions in Rust. Functions are fundamental building blocks in Rust, allowing you to organize and reuse code effectively. Understanding the correct syntax, naming conventions, and the difference between expressions and statements is essential for writing efficient Rust programs.

By mastering these concepts, you will be better equipped to handle more complex programming challenges in Rust. Continue practicing and exploring the language to deepen your understanding.
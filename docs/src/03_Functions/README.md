# Functions

## Overview

In this lesson, we will explore functions in Rust, a fundamental aspect of the language that allows for code modularity, reusability, and clarity. Functions in Rust are defined using the `fn` keyword and are essential for organizing logic and operations within your programs. We will cover function syntax, parameter passing, return values, and the distinction between expressions and statements.

## 1. Introduction to Functions in Rust

### 1.1 The `main` Function

Every Rust program starts execution from the `main` function, which serves as the entry point. This function must be named `main` and is required in all executable Rust projects. Renaming or omitting this function will result in compilation errors, as Rust will not know where to begin execution.

#### Example: Basic `main` Function

```rust
fn main() {
    println!("Hello, world!");
}
```

#### Explanation

- The `main` function in this example prints "Hello, world!" to the console. This function is automatically recognized and executed by the Rust compiler when you run your program.

### 1.2 Defining Functions

Functions in Rust are defined using the `fn` keyword followed by the function name, parameter list (if any), and the function body enclosed in curly braces. Function names should be written in snake_case, where all letters are lowercase and words are separated by underscores.

#### Example: Defining a Function

```rust
fn hello_rust() {
    println!("Hello, Rust!");
}
```

#### Explanation

- The function `hello_rust` prints "Hello, Rust!" when called. It follows Rust's naming convention of snake_case for function names.

### 1.3 Calling Functions

Once a function is defined, it can be called from anywhere within the scope where it is visible. Functions can be called before or after their definition, thanks to Rust’s feature of function hoisting, which allows functions to be defined in any order within the same scope.

#### Example: Calling a Function

```rust
fn main() {
    hello_rust();
}

fn hello_rust() {
    println!("Hello, Rust!");
}
```

#### Explanation

- The `hello_rust` function is called from the `main` function. Even though `hello_rust` is defined after the `main` function, Rust allows it due to hoisting.

## 2. Function Parameters and Return Values

### 2.1 Function Parameters

Functions in Rust can accept parameters, which are values passed to the function when it is called. Parameters are specified within the parentheses following the function name, along with their data types. Multiple parameters are separated by commas.

#### Example: Function with Parameters

```rust
fn tell_height(height: i32) {
    println!("My height is {} cm.", height);
}
```

#### Explanation

- The `tell_height` function takes an `i32` parameter named `height` and prints it as part of a formatted string.

### 2.2 Function Return Values

Functions in Rust can also return values. The return type is specified after an arrow `->` following the parameter list. The return value is usually the result of an expression within the function.

#### Example: Function Returning a Value

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

#### Explanation

- The `add` function takes two `i32` parameters, `a` and `b`, and returns their sum. Notice that the final expression `a + b` does not end with a semicolon, indicating that it is the return value of the function.

### 2.3 Expressions vs. Statements

In Rust, an **expression** is anything that returns a value, while a **statement** is a command that performs an action but does not return a value. Understanding the difference is crucial for writing correct and idiomatic Rust code.

#### Example: Expression and Statement

```rust
fn main() {
    let x = {
        let price = 5;
        let quantity = 10;
        price * quantity
    };
    println!("Result: {}", x);
}
```

#### Explanation

- The block `{ let price = 5; let quantity = 10; price * quantity }` is an expression that evaluates to `50`. The result is assigned to `x`.
- A statement would be `let price = 5;`, which does not return a value but performs an action (declaring a variable).

## 3. Advanced Function Features

### 3.1 Function with Multiple Parameters and Return Value

Rust functions can take multiple parameters of different types and return a value. This allows for more complex operations within a single function.

#### Example: Function with Multiple Parameters

```rust
fn human_id(name: &str, age: u32, height: f32) {
    println!("My name is {}, I am {} years old, and my height is {} cm.", name, age, height);
}
```

#### Explanation

- The `human_id` function takes three parameters: `name` (a string slice), `age` (an unsigned 32-bit integer), and `height` (a 32-bit floating point number). It prints these values in a formatted string.

### 3.2 Returning Values from Functions

Functions in Rust can perform operations and return the result. For example, you might want to create a function that calculates the Body Mass Index (BMI).

#### Example: Calculating BMI

```rust
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}

fn main() {
    let bmi = calculate_bmi(70.0, 1.75);
    println!("Your BMI is {:.2}", bmi);
}
```

#### Explanation

- The `calculate_bmi` function takes weight and height as parameters, computes the BMI, and returns the result. The result is printed with two decimal places.

## 4. Summary

Functions are a critical component in Rust, allowing for modular, reusable code. This lesson covered:

- The basic structure of functions in Rust.
- How to define and call functions.
- The use of parameters and return values.
- The distinction between expressions and statements.
- Advanced features like functions with multiple parameters and return values.

By mastering these concepts, you can write more organized and efficient Rust programs. Subsequent lessons will explore more advanced topics such as error handling, lifetimes, and ownership in Rust.

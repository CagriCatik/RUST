# Constants

## Overview

In this lesson, we will explore constants in Rust, which are distinct from variables and play a crucial role in defining fixed values that cannot be changed during the execution of a program. Constants, like immutable variables, are bound to a name and remain unchanged. However, constants have specific characteristics and rules that differentiate them from variables. Understanding these differences is essential for writing clear and efficient Rust code.

## 1. Introduction to Constants in Rust

### 1.1 What is a Constant?

A constant in Rust is a value that is bound to a name and is immutable, meaning it cannot be changed after its declaration. Constants are particularly useful for values that should remain consistent throughout the execution of a program, such as mathematical constants or configuration parameters.

### 1.2 Differences Between Constants and Variables

Although both constants and variables can be immutable, there are key differences between them:

- **Constants cannot be made mutable**: Unlike variables, you cannot use the `mut` keyword with constants. They are always immutable by default.
- **Type annotation is required**: When declaring a constant, you must specify its type explicitly.
- **Constants are evaluated at compile time**: The value of a constant must be a constant expression that the compiler can evaluate.
- **Scope**: Constants can be declared in any scope, including global scope, making them accessible throughout your program.

## 2. Declaring Constants in Rust

### 2.1 Syntax for Constants

Constants in Rust are declared using the `const` keyword, followed by the name of the constant, the type annotation, and the value. The name of a constant should be written in uppercase with underscores separating words, adhering to Rust's naming conventions.

#### Example: Declaring a Constant

```rust
fn main() {
    const Y: i32 = 10; // Declaring a constant `Y` with type i32 and value 10
    
    println!("The value of Y is: {}", Y);
}
```

#### Explanation

- `const Y: i32 = 10;` declares a constant `Y` of type `i32` with a value of `10`.
- The constant `Y` is used in the `println!` macro to display its value.

### 2.2 Constants Cannot Be Mutable

One of the primary rules for constants is that they cannot be declared as mutable. Any attempt to use the `mut` keyword with a constant will result in a compilation error.

#### Example: Attempting to Make a Constant Mutable

```rust
fn main() {
    const mut X: i32 = 5; // ERROR: Cannot use `mut` with a constant
}
```

#### Compilation Error

```
error: consts cannot be mutable
 --> src/main.rs:2:10
  |
2 |     const mut X: i32 = 5;
  |          ^^^
```

### 2.3 Type Annotations Are Mandatory

Unlike variables, where type inference is often sufficient, constants require explicit type annotations. The type must be specified when declaring a constant.

#### Example: Missing Type Annotation

```rust
fn main() {
    const Z = 3.14; // ERROR: Missing type for `const` item
}
```

#### Fix

```rust
fn main() {
    const Z: f64 = 3.14; // Correct: Type annotation is provided
}
```

#### Explanation

- `const Z: f64 = 3.14;` correctly declares the constant `Z` with the `f64` type.

### 2.4 Constants in Global Scope

Constants can be declared in the global scope, outside of the `main` function or any other function. This makes the constant accessible throughout the entire program.

#### Example: Global Scope Constant

```rust
const PI: f64 = 3.14;

fn main() {
    println!("The value of PI is: {}", PI);
}
```

#### Explanation

- `PI` is a constant declared in the global scope, making it accessible within the `main` function.

## 3. Practical Examples

### 3.1 Example: Defining a Constant in Global Scope

```rust
const HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // 3 hours in seconds

fn main() {
    println!("3 hours in seconds is: {}", HOURS_IN_SECONDS);
}
```

#### Explanation

- `HOURS_IN_SECONDS` is a constant that calculates the number of seconds in 3 hours. This constant is defined globally and is accessible anywhere in the program.
- The `println!` macro is used to display the calculated value.

### 3.2 Example: Using Constants Inside Functions

```rust
fn main() {
    const MAX_POINTS: u32 = 100_000;
    
    println!("The maximum points are: {}", MAX_POINTS);
}
```

#### Explanation

- `MAX_POINTS` is a constant defined inside the `main` function. Constants can be declared at any scope level, including within functions, but their immutability and type requirements remain.

## 4. Summary

In this lesson, we explored the concept of constants in Rust. Key takeaways include:

- **Immutability**: Constants are immutable and cannot be changed after they are declared.
- **Mandatory Type Annotations**: Unlike variables, constants require explicit type annotations.
- **Global Scope**: Constants can be declared globally, making them accessible throughout the entire program.
- **No Mutability**: Constants cannot be made mutable; any attempt to do so will result in a compilation error.

Understanding the rules and best practices for using constants is crucial for writing clear and efficient Rust code. Constants are particularly useful for values that need to remain consistent and unchanged throughout the execution of a program. In the next lesson, we will discuss the concept of shadowing in Rust and how it differs from mutability.

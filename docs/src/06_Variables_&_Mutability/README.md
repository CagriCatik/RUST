# Variables & Mutability

## Overview

In this lesson, we will delve into the concepts of variables and mutability in Rust. Understanding how variables work and how mutability is handled is fundamental to programming in Rust. Unlike many other languages, Rust variables are **immutable by default**, meaning once a variable is assigned a value, it cannot be changed unless explicitly made mutable. This default immutability is a key feature of Rust’s design, promoting safety and concurrency by preventing accidental modifications to data.

## 1. Variables in Rust

### 1.1 Immutability by Default

In Rust, variables are immutable by default. This means that once a value is assigned to a variable, that value cannot be changed. Immutability helps prevent bugs that arise from unintended changes to data and supports safe concurrency by ensuring that data does not change unexpectedly.

#### Example: Immutable Variable

```rust
fn main() {
    let a: u16 = 5; // Declaring an immutable variable `a` with type u16 and value 5
    
    println!("The value of a is: {}", a);
    
    // Attempting to change the value of `a` will result in a compile-time error
    // a = 10; // ERROR: Cannot assign twice to immutable variable
}
```

#### Explanation:
- The variable `a` is declared as immutable with the `let` keyword. Its type is `u16`, which is an unsigned 16-bit integer, and it is initialized with the value `5`.
- The attempt to reassign `a` to `10` will result in a compilation error because `a` is immutable.

### 1.2 Mutability in Rust

To allow a variable to be modified after its initial assignment, you must explicitly declare it as mutable using the `mut` keyword. This signals to Rust that the variable can change, which is useful in scenarios where you need to update the value of a variable as the program executes.

#### Example: Mutable Variable

```rust
fn main() {
    let mut a: u16 = 5; // Declaring a mutable variable `a`
    
    println!("The initial value of a is: {}", a);
    
    a = 10; // Reassigning a new value to `a`
    
    println!("The new value of a is: {}", a);
}
```

#### Explanation:
- By adding the `mut` keyword, the variable `a` is made mutable, allowing its value to be changed after its initial assignment.
- `a` is first assigned the value `5`, then reassigned to `10`. Both assignments are valid, and the program will compile and run without errors.

## 2. Type Annotations and Inference

Rust supports type annotations, where you explicitly specify the type of a variable. However, Rust also has strong type inference, which means it can often determine the type of a variable based on the value you assign to it.

### 2.1 Type Annotations

You can explicitly specify the type of a variable when declaring it. This is useful for clarity or when Rust cannot infer the type.

#### Example: Type Annotations

```rust
fn main() {
    let a: u16 = 5; // Type annotation specifying that `a` is a u16
    let b: f64 = 3.14; // Type annotation specifying that `b` is a 64-bit floating point
}
```

### 2.2 Type Inference

Rust’s type inference system is powerful, allowing you to omit the type in many cases. Rust will automatically infer the type based on the assigned value.

#### Example: Type Inference

```rust
fn main() {
    let a = 5; // Rust infers `a` as an i32 by default
    let b = 3.14; // Rust infers `b` as an f64 by default
}
```

#### Explanation:
- In the absence of an explicit type, Rust infers that `a` is an `i32` (the default integer type) and `b` is an `f64` (the default floating-point type).

## 3. Common Errors and Fixes

### 3.1 Immutable Variable Reassignment

Attempting to reassign a value to an immutable variable will result in a compilation error. Rust’s error messages are informative and often suggest solutions, such as making the variable mutable.

#### Example: Compilation Error

```rust
fn main() {
    let a = 5;
    a = 10; // ERROR: Cannot assign twice to immutable variable `a`
}
```

#### Fix:

```rust
fn main() {
    let mut a = 5; // Fix by making `a` mutable
    a = 10;
}
```

### 3.2 Unused Variables

Rust warns you if you declare a variable but do not use it. This is to help you avoid having unused code that could indicate a logical error in your program.

#### Example: Unused Variable Warning

```rust
fn main() {
    let a = 5; // Warning: unused variable `a`
}
```

#### Fix:
- To suppress the warning, you can either use the variable or prefix the variable name with an underscore (`_`).

```rust
fn main() {
    let _a = 5; // No warning since `_a` is intentionally unused
}
```

## 4. Summary

In this lesson, we explored variables and mutability in Rust. Key points include:
- **Immutability by Default**: Variables in Rust are immutable unless explicitly marked as mutable with the `mut` keyword.
- **Mutability**: Mutable variables can be reassigned, allowing for changes to their value after initial assignment.
- **Type Annotations and Inference**: Rust can infer types based on assigned values, but you can also explicitly specify types for clarity or when necessary.
- **Common Errors**: Rust’s compiler provides helpful error messages and suggestions to fix issues related to immutability and unused variables.

Understanding these concepts is crucial for writing safe, efficient, and idiomatic Rust code. In the next lesson, we will explore constants in Rust, which differ from variables in important ways, particularly regarding mutability and scope.
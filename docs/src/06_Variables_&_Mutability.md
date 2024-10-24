# Variables & Mutability in Rust

## Overview

Understanding how variables work and how mutability is handled is fundamental to programming in Rust. Unlike many other languages, Rust variables are **immutable by default**, meaning once a variable is assigned a value, it cannot be changed unless explicitly made mutable. This default immutability is a key feature of Rust’s design, promoting safety and concurrency by preventing accidental modifications to data. This lesson will cover the basics of declaring variables, mutability, type annotations, type inference, and common errors related to variables and mutability in Rust.

---

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

**Output:**
```
The value of a is: 5
```

#### Explanation

- The variable `a` is declared as immutable with the `let` keyword. Its type is `u16`, which is an unsigned 16-bit integer, and it is initialized with the value `5`.
- The attempt to reassign `a` to `10` (currently commented out) will result in a compilation error because `a` is immutable.
- This immutability ensures that once `a` is set, its value remains constant throughout its scope, preventing accidental modifications.

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

**Output:**
```
The initial value of a is: 5
The new value of a is: 10
```

#### Explanation

- By adding the `mut` keyword, the variable `a` is made mutable, allowing its value to be changed after its initial assignment.
- `a` is first assigned the value `5`, then reassigned to `10`. Both assignments are valid, and the program will compile and run without errors.
- Mutable variables provide the flexibility to update data as needed while still adhering to Rust's safety guarantees.

---

## 2. Type Annotations and Inference

Rust is a statically-typed language, meaning that the type of every variable must be known at compile time. Rust provides two ways to specify variable types: **type annotations** and **type inference**. Understanding these mechanisms is essential for writing clear and efficient Rust code.

### 2.1 Type Annotations

You can explicitly specify the type of a variable when declaring it. This is useful for clarity, documentation, or when Rust cannot infer the type on its own.

#### Example: Type Annotations

```rust
fn main() {
    let a: u16 = 5; // Type annotation specifying that `a` is a u16
    let b: f64 = 3.14; // Type annotation specifying that `b` is a 64-bit floating point
    
    println!("a: {}, b: {}", a, b);
}
```

**Output:**
```
a: 5, b: 3.14
```

#### Explanation

- The variable `a` is explicitly annotated as a `u16`, an unsigned 16-bit integer, and initialized with the value `5`.
- The variable `b` is explicitly annotated as an `f64`, a 64-bit floating-point number, and initialized with the value `3.14`.
- Type annotations improve code readability and are necessary when Rust cannot infer the type based on the context.

### 2.2 Type Inference

Rust’s type inference system is powerful, allowing you to omit the type in many cases. Rust will automatically infer the type based on the assigned value, reducing the need for repetitive type annotations and making the code cleaner.

#### Example: Type Inference

```rust
fn main() {
    let a = 5; // Rust infers `a` as an i32 by default
    let b = 3.14; // Rust infers `b` as an f64 by default
    
    println!("a: {}, b: {}", a, b);
}
```

**Output:**
```
a: 5, b: 3.14
```

#### Explanation

- In the absence of an explicit type, Rust infers that `a` is an `i32` (the default integer type) and `b` is an `f64` (the default floating-point type).
- Type inference reduces verbosity, allowing developers to write concise code without sacrificing type safety.
- While type inference is convenient, explicit type annotations are still important for clarity and when dealing with complex types or when the inferred type is not obvious.

---

## 3. Common Errors and Fixes

Rust's strict type and mutability rules help prevent many common programming errors. Understanding these rules and the corresponding compiler errors is crucial for effective Rust programming. Below are some typical issues developers might encounter related to variables and mutability, along with explanations and solutions.

### 3.1 Immutable Variable Reassignment

Attempting to reassign a value to an immutable variable will result in a compilation error. Rust’s error messages are informative and often suggest solutions, such as making the variable mutable.

#### Example: Compilation Error

```rust
fn main() {
    let a = 5;
    a = 10; // ERROR: Cannot assign twice to immutable variable `a`
}
```

**Compiler Error:**
```
error[E0384]: cannot assign twice to immutable variable `a`
 --> src/main.rs:3:5
  |
2 |     let a = 5;
  |         - first assignment to `a`
3 |     a = 10; // ERROR: Cannot assign twice to immutable variable `a`
  |     ^^^^^ cannot assign twice to immutable variable
```

#### Fix:

To fix this error, you need to declare the variable as mutable using the `mut` keyword.

```rust
fn main() {
    let mut a = 5; // Fix by making `a` mutable
    a = 10;
    println!("The value of a is: {}", a);
}
```

**Output:**
```
The value of a is: 10
```

#### Explanation

- The original code attempts to reassign `a` after it has been declared immutable, resulting in a compile-time error.
- By declaring `a` as mutable (`let mut a = 5;`), you inform Rust that `a` can be changed, allowing the reassignment to `10` without errors.

### 3.2 Unused Variables

Rust warns you if you declare a variable but do not use it. This is to help you avoid having unused code that could indicate a logical error in your program.

#### Example: Unused Variable Warning

```rust
fn main() {
    let a = 5; // Warning: unused variable `a`
}
```

**Compiler Warning:**
```
warning: unused variable: `a`
 --> src/main.rs:2:9
  |
2 |     let a = 5; // Warning: unused variable `a`
  |         ^ help: if this is intentional, prefix it with an underscore: `_a`
  |
  = note: `#[warn(unused_variables)]` on by default
```

#### Fix:

To suppress the warning, you can either use the variable or prefix the variable name with an underscore (`_`).

**Using the Variable:**

```rust
fn main() {
    let a = 5;
    println!("The value of a is: {}", a); // Now `a` is used
}
```

**Output:**
```
The value of a is: 5
```

**Using an Underscore Prefix:**

```rust
fn main() {
    let _a = 5; // No warning since `_a` is intentionally unused
}
```

**Explanation**

- In the first fix, by using `a` in a `println!` statement, the compiler recognizes that the variable is being used, eliminating the warning.
- In the second fix, prefixing the variable name with an underscore (`_a`) indicates to the compiler that the variable is intentionally unused, suppressing the warning.
- These practices help maintain clean and intentional code, preventing the accumulation of unnecessary or unused variables.

---

## 4. Summary

In this lesson, we explored variables and mutability in Rust. Key points include:

- **Immutability by Default**: Variables in Rust are immutable unless explicitly marked as mutable with the `mut` keyword.
- **Mutability**: Mutable variables can be reassigned, allowing for changes to their value after initial assignment.
- **Type Annotations and Inference**: Rust can infer types based on assigned values, but you can also explicitly specify types for clarity or when necessary.
- **Common Errors**: Rust’s compiler provides helpful error messages and suggestions to fix issues related to immutability and unused variables.

Understanding these concepts is crucial for writing safe, efficient, and idiomatic Rust code. In the next lesson, we will explore **Constants** in Rust, which differ from variables in important ways, particularly regarding mutability and scope.

### Key Takeaways

- **Immutability Enhances Safety**: By default, variables are immutable, preventing accidental changes and promoting safer code.
- **Explicit Mutability**: The `mut` keyword allows controlled mutability, ensuring that only intended variables can be changed.
- **Type Safety**: Rust's ability to infer types reduces verbosity, while type annotations provide clarity and handle complex scenarios.
- **Compiler Assistance**: Rust's compiler catches common mistakes early, guiding developers towards writing correct and efficient code.

### Next Steps

Building upon your understanding of variables and mutability, future lessons will delve into:

- **Constants and Static Variables**: Exploring how to define and use constants, which are different from variables in terms of mutability and scope.
- **Data Types**: Deepening knowledge of Rust’s data types, including compound types like arrays, tuples, slices, and strings.
- **Ownership and Borrowing**: Understanding Rust's unique ownership model and how borrowing and references work to ensure memory safety.
- **Error Handling**: Learning how to handle errors gracefully using Rust’s `Result` and `Option` types.
- **Concurrency**: Leveraging Rust’s concurrency features to write safe and efficient multi-threaded programs.

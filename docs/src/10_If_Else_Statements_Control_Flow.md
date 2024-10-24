# If Else Statements / Control Flow

## Overview

In this lesson, we will explore control flow in Rust, focusing on `if-else` expressions. Control flow is an essential concept in programming, determining how the program executes different blocks of code based on conditions. In Rust, as in other programming languages, `if-else` expressions allow you to branch your code depending on whether certain conditions are true or false. Mastering control flow is crucial for building complex and responsive applications.

## 1. Introduction to Control Flow

### 1.1 What is Control Flow?

Control flow refers to the order in which individual statements, instructions, or function calls are executed or evaluated in a programming language. In Rust, control flow is primarily managed through:
- **Conditions**: Checking if certain criteria are met.
- **Repeating actions**: Looping over blocks of code based on conditions.

In this lesson, we will focus on how conditions are handled in Rust using `if-else` expressions.

### 1.2 If-Else Expressions

An `if` expression in Rust allows you to execute a block of code based on whether a condition evaluates to true. The `else` clause provides an alternative block of code to execute if the condition is false.

#### Example: Basic If-Else Expression

```rust
fn main() {
    let age = 18;

    if age >= 18 {
        println!("You can drive a car.");
    } else {
        println!("You are not old enough to drive.");
    }
}
```

#### Explanation:
- The condition `age >= 18` is checked.
- If the condition is true, the program prints "You can drive a car."
- If the condition is false, it prints "You are not old enough to drive."

## 2. Using If-Else Statements

### 2.1 Simple If Statement

The most basic control flow statement is the `if` statement. It checks a condition and executes the associated block of code if the condition is true.

#### Example: Simple If Statement

```rust
fn main() {
    let temperature = 30;

    if temperature > 25 {
        println!("It's a hot day!");
    }
}
```

#### Explanation:
- The condition `temperature > 25` is evaluated.
- Since the temperature is 30, which is greater than 25, the program prints "It's a hot day!"

### 2.2 If-Else Statement

An `if-else` statement allows you to specify an alternative block of code to execute when the condition is false.

#### Example: If-Else Statement

```rust
fn main() {
    let age = 16;

    if age >= 18 {
        println!("You can drive a car.");
    } else {
        println!("You are not old enough to drive.");
    }
}
```

#### Explanation:
- The condition `age >= 18` is false because the age is 16.
- The `else` block is executed, printing "You are not old enough to drive."

### 2.3 Multiple Conditions with Else If

Sometimes, you need to check multiple conditions. This is where `else if` comes in handy. It allows you to test additional conditions if the previous ones are false.

#### Example: Else If Statement

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("The number is divisible by 4.");
    } else if number % 3 == 0 {
        println!("The number is divisible by 3.");
    } else if number % 2 == 0 {
        println!("The number is divisible by 2.");
    } else {
        println!("The number is not divisible by 4, 3, or 2.");
    }
}
```

#### Explanation:
- The program checks if `number` is divisible by 4, then by 3, and then by 2.
- Since 6 is divisible by 3, the corresponding block of code is executed, and "The number is divisible by 3." is printed.

## 3. If in a Let Statement

In Rust, you can use `if-else` expressions to assign values to variables. This can be particularly useful for making decisions within a single line of code.

#### Example: If-Else in a Let Statement

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The number is: {}", number);
}
```

#### Explanation:
- The variable `number` is assigned the value `5` if `condition` is true, or `6` if `condition` is false.
- Since `condition` is true, `number` is set to `5`, and the program prints "The number is: 5."

### 3.1 Ensuring Compatible Types

It is important to ensure that both branches of an `if-else` expression return values of the same type. If the types are incompatible, Rust will produce a compilation error.

#### Example: Incompatible Types in If-Else

```rust
fn main() {
    let condition = false;
    let number = if condition { 5 } else { "six" }; // ERROR: Incompatible types

    println!("The number is: {}", number);
}
```

#### Compilation Error:
```
error[E0308]: if and else have incompatible types
 --> src/main.rs:3:34
  |
3 |     let number = if condition { 5 } else { "six" };
  |                                  ^ expected integer, found `&str`
```

#### Explanation:
- The `if` branch returns an integer, while the `else` branch returns a string. Rust requires both branches to return values of the same type, so this code results in a compilation error.

## 4. Summary

In this lesson, we covered the basics of control flow in Rust, focusing on `if-else` expressions. Key points include:
- **If Statements**: Execute a block of code if a condition is true.
- **If-Else Statements**: Provide an alternative block of code if the condition is false.
- **Else If Statements**: Check additional conditions when the previous conditions are false.
- **If in Let Statements**: Assign values based on conditions directly within a `let` statement.
- **Type Compatibility**: Ensure that all branches of an `if-else` expression return the same type to avoid compilation errors.

Understanding and effectively using control flow is essential for writing flexible and responsive Rust programs. In the next lesson, we will explore loops, another critical aspect of control flow, which allows for repeating actions based on conditions.
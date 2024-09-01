# Loops / Control Flow

## Overview

In this lesson, we will explore the concept of loops in Rust, a fundamental aspect of control flow. Loops allow you to execute a block of code multiple times, which is crucial for tasks that require repetition, such as iterating over a collection or retrying an operation until a condition is met. Rust provides three primary loop constructs: `loop`, `while`, and `for`. Each of these loops serves different purposes and offers unique control over the flow of your program.

## 1. Types of Loops in Rust

### 1.1 `loop` - The Infinite Loop

The `loop` keyword in Rust creates an unconditional loop that will run indefinitely unless explicitly stopped using the `break` statement. This type of loop is useful when you want to repeat a block of code until a specific condition is met from within the loop.

#### Example: Basic Infinite Loop

```rust
fn main() {
    loop {
        println!("Hello, world!");
    }
}
```

#### Explanation:
- The `loop` will continuously print "Hello, world!" until manually interrupted (e.g., by pressing `Ctrl+C` in the terminal).

### 1.2 Breaking a Loop

To stop an infinite loop, you use the `break` statement. This is often paired with a conditional check to exit the loop when a certain condition is met.

#### Example: Loop with Break

```rust
fn main() {
    let mut counter = 0;

    loop {
        counter += 1;

        if counter == 10 {
            break;
        }
    }

    println!("Counter reached: {}", counter);
}
```

#### Explanation:
- The loop increments `counter` by 1 on each iteration.
- When `counter` reaches 10, the `break` statement exits the loop, and the final value of `counter` is printed.

### 1.3 Returning Values from Loops

Rust allows you to return a value from a loop by placing the value after the `break` statement. This can be useful when you need to perform an operation repeatedly until a desired result is achieved.

#### Example: Returning a Value from a Loop

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);
}
```

#### Explanation:
- The loop runs until `counter` equals 10.
- When the loop breaks, it returns `counter * 2`, which is 20 in this case, and assigns it to `result`.

## 2. Loop Labels

### 2.1 Nested Loops and Labels

When working with nested loops, it can become unclear which loop a `break` or `continue` statement is referring to. Rust provides loop labels to clarify this by explicitly naming loops and specifying which loop should be affected by `break` or `continue`.

#### Example: Using Loop Labels

```rust
fn main() {
    let mut count = 0;

    'outer: loop {
        println!("Count = {}", count);
        let mut remaining = 10;

        loop {
            println!("Remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'outer;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End of loop with count = {}", count);
}
```

#### Explanation:
- The `outer` loop is labeled with `'outer:`.
- The inner loop decreases `remaining` until it equals 9, then breaks out of the inner loop.
- When `count` equals 2, the program breaks out of the `outer` loop using the `'outer` label.

## 3. `while` Loops

### 3.1 Conditional Loops

The `while` loop runs as long as a specified condition is true. This loop is useful for scenarios where the number of iterations is not known beforehand, and the loop needs to continue until a condition changes.

#### Example: While Loop

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("Liftoff!");
}
```

#### Explanation:
- The `while` loop runs as long as `number` is not equal to 0.
- Each iteration prints the current value of `number` and then decrements it by 1.
- When `number` reaches 0, the loop exits and "Liftoff!" is printed.

## 4. `for` Loops

### 4.1 Iterating Over a Collection

The `for` loop in Rust is used to iterate over collections such as arrays or ranges. It automatically handles iteration and avoids the common pitfalls associated with manually managing loop counters.

#### Example: Iterating Over an Array

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    for element in a {
        println!("The value is: {}", element);
    }
}
```

#### Explanation:
- The `for` loop iterates over each element in the array `a`.
- Each value is printed during its respective iteration.

### 4.2 Iterating Over a Range

Rust’s `for` loop can also iterate over a range of numbers, which is especially useful for counting up or down.

#### Example: Iterating Over a Range

```rust
fn main() {
    for number in 1..4 {
        println!("The number is: {}", number);
    }
}
```

#### Explanation:
- The `for` loop iterates over the range `1..4`, printing the numbers 1 through 3.
- Note that the range `1..4` is exclusive of 4.

## 5. Summary

In this lesson, we explored the different types of loops in Rust, each serving a unique purpose:
- **`loop`**: An unconditional loop that continues until explicitly stopped using `break`.
- **`while`**: A conditional loop that continues as long as a condition is true.
- **`for`**: A loop that iterates over collections or ranges, automatically handling iteration logic.

Understanding how and when to use each type of loop is crucial for controlling the flow of your Rust programs effectively. These loops are powerful tools for iterating over data, performing repetitive tasks, and managing complex control flows in your applications. In the next lesson, we will delve deeper into more advanced Rust features that build on these foundational concepts.
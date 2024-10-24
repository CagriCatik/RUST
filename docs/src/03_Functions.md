#  Functions in Rust

## Overview
Functions are a fundamental aspect of the Rust programming language, enabling code modularity, reusability, and clarity. By encapsulating logic within functions, you can organize your code more effectively, reduce redundancy, and enhance maintainability. This lesson will cover the basics of defining and calling functions, passing parameters, returning values, and delve into more advanced features such as multiple parameters and the distinction between expressions and statements.

---

## 1. Introduction to Functions in Rust

Functions in Rust are defined using the `fn` keyword and are essential for organizing logic and operations within your programs. Understanding how to effectively use functions will allow you to write cleaner, more efficient, and more manageable Rust code.

### 1.1 The `main` Function

Every Rust program begins execution from the `main` function, which serves as the entry point. This function must be named `main` and is required in all executable Rust projects. Omitting or renaming this function will result in compilation errors, as Rust will not know where to start execution.

#### Example: Basic `main` Function

```rust
fn main() {
    println!("Hello, world!");
}
```

**Output:**
```
Hello, world!
```

#### Explanation

- The `main` function in this example prints "Hello, world!" to the console.
- This function is automatically recognized and executed by the Rust compiler when you run your program.
- The `println!` macro is used to print formatted text to the console.

### 1.2 Defining Functions

Functions in Rust are defined using the `fn` keyword, followed by the function name, parameter list (if any), and the function body enclosed in curly braces. Function names should follow Rust's naming conventions, typically using `snake_case`, where all letters are lowercase and words are separated by underscores.

#### Example: Defining a Function

```rust
fn hello_rust() {
    println!("Hello, Rust!");
}
```

**Output When Called:**
```
Hello, Rust!
```

#### Explanation

- The function `hello_rust` prints "Hello, Rust!" when called.
- It follows Rust's naming convention of using `snake_case` for function names.
- This function does not take any parameters and does not return a value.

### 1.3 Calling Functions

Once a function is defined, it can be called from anywhere within its scope. Rust allows functions to be called before or after their definition thanks to function hoisting, which permits functions to be defined in any order within the same scope.

#### Example: Calling a Function

```rust
fn main() {
    hello_rust();
}

fn hello_rust() {
    println!("Hello, Rust!");
}
```

**Output:**
```
Hello, Rust!
```

#### Explanation

- The `hello_rust` function is called from within the `main` function.
- Even though `hello_rust` is defined after the `main` function, Rust allows this due to function hoisting.
- This demonstrates that the order of function definitions does not affect their ability to be called within the same scope.

---

## 2. Function Parameters and Return Values

Functions often need to operate on data provided to them. Rust functions can accept parameters and return values, allowing for flexible and dynamic operations.

### 2.1 Function Parameters

Functions in Rust can accept parameters, which are values passed to the function when it is called. Parameters are specified within the parentheses following the function name, along with their data types. Multiple parameters are separated by commas.

#### Example: Function with Parameters

```rust
fn tell_height(height: i32) {
    println!("My height is {} cm.", height);
}

fn main() {
    tell_height(175);
}
```

**Output:**
```
My height is 175 cm.
```

#### Explanation

- The `tell_height` function takes an `i32` parameter named `height`.
- When called with the value `175`, it prints "My height is 175 cm." to the console.
- Specifying parameter types ensures type safety, a core feature of Rust's design.

### 2.2 Function Return Values

Functions in Rust can also return values. The return type is specified after an arrow `->` following the parameter list. The return value is typically the result of an expression within the function.

#### Example: Function Returning a Value

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add(5, 7);
    println!("The sum is {}.", sum);
}
```

**Output:**
```
The sum is 12.
```

#### Explanation

- The `add` function takes two `i32` parameters, `a` and `b`, and returns their sum.
- The return type `i32` is specified after the `->` symbol.
- The expression `a + b` calculates the sum and is returned implicitly because it does not end with a semicolon.
- In the `main` function, `add(5, 7)` is called, and the result is stored in the variable `sum`, which is then printed.

### 2.3 Expressions vs. Statements

In Rust, understanding the difference between **expressions** and **statements** is crucial for writing correct and idiomatic code.

- **Expressions**: These evaluate to a value and can be used wherever values are expected.
- **Statements**: These perform an action but do not return a value.

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

**Output:**
```
Result: 50
```

#### Explanation

- The block `{ let price = 5; let quantity = 10; price * quantity }` is an **expression** that evaluates to `50`.
- The result of the expression is assigned to the variable `x`.
- Statements like `let price = 5;` and `let quantity = 10;` perform actions (declaring variables) but do not return values.
- The final line `price * quantity` is an expression whose value is returned from the block.

---

## 3. Advanced Function Features

Beyond the basics, Rust offers several advanced features for functions, allowing for more complex and versatile operations.

### 3.1 Function with Multiple Parameters and Return Value

Rust functions can take multiple parameters of different types and return a value, enabling more complex operations within a single function.

#### Example: Function with Multiple Parameters

```rust
fn human_id(name: &str, age: u32, height: f32) {
    println!(
        "My name is {}, I am {} years old, and my height is {} cm.",
        name, age, height
    );
}

fn main() {
    human_id("Alice", 30, 165.5);
}
```

**Output:**
```
My name is Alice, I am 30 years old, and my height is 165.5 cm.
```

#### Explanation

- The `human_id` function takes three parameters:
  - `name`: a string slice (`&str`)
  - `age`: an unsigned 32-bit integer (`u32`)
  - `height`: a 32-bit floating-point number (`f32`)
- It prints a formatted string incorporating all three parameters.
- The function is called from `main` with the arguments `"Alice"`, `30`, and `165.5`.

### 3.2 Returning Values from Functions

Functions in Rust can perform operations and return the result. For example, you might want to create a function that calculates the Body Mass Index (BMI).

#### Example: Calculating BMI

```rust
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}

fn main() {
    let bmi = calculate_bmi(70.0, 1.75);
    println!("Your BMI is {:.2}.", bmi);
}
```

**Output:**
```
Your BMI is 22.86.
```

#### Explanation

- The `calculate_bmi` function takes two `f64` parameters: `weight_kg` and `height_m`.
- It calculates BMI using the formula: weight divided by the square of height.
- The function returns the result as an `f64`.
- In the `main` function, `calculate_bmi(70.0, 1.75)` is called, and the result is stored in `bmi`.
- The BMI value is printed with two decimal places using the formatting specifier `{:.2}`.

---

## 4. Summary

Functions are a critical component in Rust, enabling modular, reusable, and organized code. This lesson covered:

- **The Basic Structure of Functions in Rust**: Understanding how to define and name functions.
- **How to Define and Call Functions**: Learning the syntax for creating and invoking functions.
- **The Use of Parameters and Return Values**: Passing data into functions and retrieving results.
- **The Distinction Between Expressions and Statements**: Differentiating between actions and value-returning computations.
- **Advanced Features Like Functions with Multiple Parameters and Return Values**: Handling more complex scenarios with multiple inputs and outputs.

By mastering these concepts, you can write more organized and efficient Rust programs. Functions not only help in breaking down complex problems into manageable pieces but also promote code reuse and maintainability.

### Next Steps

Building upon your understanding of functions, future lessons will explore more advanced Rust concepts, including:

- **Error Handling**: Managing and responding to errors gracefully using `Result` and `Option` types.
- **Ownership and Borrowing**: Deep diving into Rust’s ownership model to ensure memory safety without a garbage collector.
- **Lifetimes**: Understanding how Rust manages the scope and validity of references.
- **Advanced Data Structures**: Exploring collections like vectors, hash maps, and custom data structures.
- **Concurrency**: Harnessing Rust’s concurrency features to write safe and efficient multi-threaded programs.

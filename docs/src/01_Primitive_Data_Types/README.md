# Primitive Data Types

## Overview

This tutorial will provide an in-depth explanation of primitive data types in the Rust programming language. Rust, a statically-typed language, requires the explicit declaration of data types for variables. Understanding these types is crucial for efficient memory management and ensuring code correctness. This lesson covers the fundamental primitive data types, including integers, floating-point numbers, Booleans, and characters.

## 1. Primitive Data Types in Rust

Rust’s primitive data types, also known as scalar types, represent single values. These types are foundational to understanding how data is handled in Rust. The four primary scalar types in Rust are:

1. **Integers**: Signed (`i8`, `i16`, `i32`, `i64`, `i128`) and Unsigned (`u8`, `u16`, `u32`, `u64`, `u128`).
2. **Floating-point numbers**: `f32` and `f64`.
3. **Boolean**: `bool`.
4. **Character**: `char`.

### 1.1 Integer Types

Rust offers both signed and unsigned integers, each with a range defined by their bit size.

- **Signed Integers (`i8`, `i16`, `i32`, `i64`, `i128`)**: These integers can store both positive and negative numbers. The range of values depends on the bit size. For example:
  - `i8` ranges from -128 to 127.
  - `i32` ranges from -2^31 to 2^31 - 1.
  
- **Unsigned Integers (`u8`, `u16`, `u32`, `u64`, `u128`)**: These integers store only positive numbers and have a range starting from 0. For example:
  - `u8` ranges from 0 to 255.
  - `u32` ranges from 0 to 2^32 - 1.

#### Example: Integer Types

```rust
fn main() {
    let x: i32 = -42; // Signed 32-bit integer
    let y: u64 = 100; // Unsigned 64-bit integer
    
    println!("Signed integer: {}", x);
    println!("Unsigned integer: {}", y);
}
```

#### Explanation

- In this example, `x` is an `i32`, allowing it to hold both positive and negative values.
- `y` is a `u64`, meaning it can only hold positive values.

### 1.2 Floating-Point Types

Floating-point numbers in Rust represent real numbers with fractional parts. Rust supports two types:

- **`f32`**: A 32-bit floating-point number.
- **`f64`**: A 64-bit floating-point number, which is the default type for floating-point literals in Rust.

#### Example: Floating-Point Types

```rust
fn main() {
    let pi: f64 = 3.14; // 64-bit floating-point number
    
    println!("Value of Pi: {}", pi);
}
```

#### Explanation

- Here, `pi` is defined as an `f64`, which provides more precision compared to `f32`.

### 1.3 Boolean Type

The Boolean type in Rust is represented by the `bool` keyword. It can take only two values: `true` or `false`.

#### Example: Boolean Type

```rust
fn main() {
    let is_snowing: bool = true;
    
    println!("Is it snowing? {}", is_snowing);
}
```

#### Explanation

- The variable `is_snowing` is of type `bool` and holds the value `true`.

### 1.4 Character Type

The `char` type in Rust represents a single Unicode scalar value. It can hold any character, including letters, numbers, and special symbols.

#### Example: Character Type

```rust
fn main() {
    let letter: char = 'A';
    
    println!("First letter of the alphabet: {}", letter);
}
```

#### Explanation

- The variable `letter` is of type `char` and holds the value `'A'`. Rust’s `char` type is 4 bytes in size and can represent a wide range of characters beyond just ASCII.

## 2. Common Errors and Considerations

Understanding the size and range of each data type is crucial for avoiding common errors:

### 2.1 Integer Overflow

If you try to assign a value outside the range of a type, Rust’s compiler will throw an error.

#### Example

```rust
fn main() {
    let x: i32 = 2_147_483_648; // This exceeds the i32 range
}
```

#### Compiler Error

```
error: literal out of range for `i32`
```

### 2.2 Type Mismatch

Rust enforces strict type checking. You cannot assign a negative value to an unsigned integer, nor can you mix types without explicit conversion.

#### Example

```rust
fn main() {
    let y: u64 = -100; // Unsigned type cannot hold a negative value
}
```

#### Compiler Error

```
error: cannot apply unary operator `-` to type `u64`
```

## 3. Summary

Rust’s primitive data types are essential for understanding how to efficiently manage memory and ensure code correctness. By mastering these types, developers can write more reliable and performant Rust code. This lesson covered:

- **Integer Types**: Understanding signed and unsigned integers and their ranges.
- **Floating-Point Types**: Handling real numbers with fractional parts.
- **Boolean Type**: Using `bool` for logical operations.
- **Character Type**: Representing single Unicode scalar values with `char`.

Further lessons will build on these concepts, delving into compound data types and more advanced Rust programming techniques.

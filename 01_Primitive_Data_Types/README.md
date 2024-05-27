# Primitive Data Types

## Overview

Rust is a statically typed programming language, which means the type of each variable must be known at compile time. This ensures type safety and can prevent a wide range of bugs. Rust's primitive data types, also known as scalar types, include integers, floating-point numbers, Booleans, and characters. This lesson covers each of these types in detail, providing examples to illustrate their usage.

## Integers

### Signed and Unsigned Integers

In Rust, integers come in two main varieties: signed and unsigned. Signed integers can represent both positive and negative values, while unsigned integers can only represent positive values. Each variety comes in different sizes, which determine the range of values they can hold.

#### Signed Integers

- `i8`: 8-bit signed integer
- `i16`: 16-bit signed integer
- `i32`: 32-bit signed integer
- `i64`: 64-bit signed integer
- `i128`: 128-bit signed integer

#### Unsigned Integers

- `u8`: 8-bit unsigned integer
- `u16`: 16-bit unsigned integer
- `u32`: 32-bit unsigned integer
- `u64`: 64-bit unsigned integer
- `u128`: 128-bit unsigned integer

### Example

```rust
fn main() {
    let x: i32 = 42; // Signed 32-bit integer
    let y: u64 = 100; // Unsigned 64-bit integer
    
    println!("Signed integer: {}", x);
    println!("Unsigned integer: {}", y);
}
```

### Range of Values

The range of values an integer type can hold depends on its size. For example, an `i32` can hold values from `-2^31` to `2^31 - 1`, while a `u32` can hold values from `0` to `2^32 - 1`.

```rust
fn main() {
    let max_i32: i32 = i32::MAX;
    let max_i64: i64 = i64::MAX;
    
    println!("Maximum value of i32: {}", max_i32);
    println!("Maximum value of i64: {}", max_i64);
}
```

### Compiler Errors

If you try to assign a value that is out of range for a given type, the Rust compiler will produce an error.

```rust
fn main() {
    let x: i32 = 2_147_483_648; // This will cause a compile-time error
}
```

The error message will indicate that the value does not fit within the specified type.

## Floating-Point Numbers

Rust provides two types of floating-point numbers: `f32` and `f64`. These types represent numbers with fractional parts.

### Example

```rust
fn main() {
    let pi: f64 = 3.14;
    
    println!("Value of Pi: {}", pi);
}
```

## Boolean Values

Boolean values in Rust are represented by the `bool` type. A `bool` can have one of two values: `true` or `false`.

### Example

```rust
fn main() {
    let is_snowing: bool = true;
    
    println!("Is it snowing? {}", is_snowing);
}
```

## Characters

The `char` type represents a single Unicode scalar value. This includes characters from many languages, as well as special characters.

### Example

```rust
fn main() {
    let letter: char = 'A';
    
    println!("First letter of the alphabet: {}", letter);
}
```

## Summary

This lesson covered the basic primitive data types in Rust, including integers, floating-point numbers, Booleans, and characters. Understanding these types is fundamental to writing safe and efficient Rust programs.
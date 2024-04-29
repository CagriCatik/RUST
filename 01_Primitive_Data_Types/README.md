# Primitive Data Types

In this tutorial, we'll delve into the fundamental building blocks of Rust programming language: the primitive data types. Rust, being a statically typed language, requires you to specify the data type of variables during declaration. Primitive data types, also known as scalar data types, include integers, floating-point numbers, Booleans, and characters.

## 1. Integers

Rust provides both signed and unsigned integer types of various sizes. Signed integers can hold positive or negative values, while unsigned integers can only hold positive values.

### Code Example:

```rust
fn main() {
    let x: i32 = 42; // Signed integer
    let y: u64 = 100; // Unsigned integer

    println!("Signed integer: {}", x);
    println!("Unsigned integer: {}", y);
}
```

### Explanation:

- Signed integers: `i8`, `i16`, `i32`, `i64`, `i128`
- Unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128`

The size and range of values vary with the type (`i32` can hold 32 bits, while `u64` can hold 64 bits). The range for `i32` is from `-2^31` to `2^31 - 1`, and for `i64` it's from `-2^63` to `2^63 - 1`.

## 2. Floating-Point Numbers

Floating-point numbers represent numbers with fractional parts. Rust offers two floating-point types: `f32` and `f64`.

### Code Example:

```rust
fn main() {
    let pi: f64 = 3.14;

    println!("Value of Pi: {}", pi);
}
```

### Explanation:

- `f32`: 32-bit floating point
- `f64`: 64-bit floating point

## 3. Booleans

Boolean values can be either `true` or `false`.

### Code Example:

```rust
fn main() {
    let is_snowing: bool = true;

    println!("Is it snowing? {}", is_snowing);
}
```

### Explanation:

- `bool`: Boolean type, values can be `true` or `false`.

## 4. Characters

Characters represent single Unicode scalar values.

### Code Example:

```rust
fn main() {
    let letter: char = 'a';

    println!("First letter of the alphabet is: {}", letter);
}
```

### Explanation:

- `char`: Character type, representing a single Unicode scalar value.

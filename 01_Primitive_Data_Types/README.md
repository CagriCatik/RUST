# Primitive Data Types

## Introduction

In Rust, understanding primitive data types is fundamental to mastering the language. Rust is a statically typed programming language, which means you must declare the data type of each variable explicitly. The primitive data types in Rust, also known as scalar types, include integers, floating-point numbers, Booleans, and characters. This tutorial will delve into each of these types, exploring their properties, ranges, and usage.

## Integers

### Signed and Unsigned Integers

Rust provides both signed and unsigned integers of various sizes. The primary difference between these two types is that signed integers can hold both positive and negative values, whereas unsigned integers can only hold non-negative values. Here are the integer types available in Rust:

- **Signed Integers**: `i8`, `i16`, `i32`, `i64`, `i128`
- **Unsigned Integers**: `u8`, `u16`, `u32`, `u64`, `u128`

### Example: Signed and Unsigned Integers

```rust
fn main() {
    let x: i32 = 42;  // Signed 32-bit integer
    let y: u64 = 100; // Unsigned 64-bit integer
    
    println!("Signed integer: {}", x);
    println!("Unsigned integer: {}", y);
}
```

In this example, `x` is a signed 32-bit integer, meaning it can hold values from -2^31 to 2^31 - 1. On the other hand, `y` is an unsigned 64-bit integer, capable of holding values from 0 to 2^64 - 1.

### Range and Size

The size of the integer determines the range of values it can hold:

- `i8`: -128 to 127
- `u8`: 0 to 255
- `i16`: -32,768 to 32,767
- `u16`: 0 to 65,535
- `i32`: -2,147,483,648 to 2,147,483,647
- `u32`: 0 to 4,294,967,295
- `i64`: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
- `u64`: 0 to 18,446,744,073,709,551,615
- `i128`: -170,141,183,460,469,231,731,687,303,715,884,105,728 to 170,141,183,460,469,231,731,687,303,715,884,105,727
- `u128`: 0 to 340,282,366,920,938,463,463,374,607,431,768,211,455

### Example: Maximum Values

```rust
fn main() {
    let max_i32: i32 = i32::MAX;
    let max_i64: i64 = i64::MAX;
    
    println!("Maximum value of i32: {}", max_i32);
    println!("Maximum value of i64: {}", max_i64);
}
```

In this example, `i32::MAX` and `i64::MAX` are used to obtain the maximum values for `i32` and `i64` types, respectively.

## Floating-Point Numbers

Rust supports two types of floating-point numbers, which are used to represent numbers with fractional parts:

- `f32`: 32-bit floating-point number
- `f64`: 64-bit floating-point number

### Example: Floating-Point Numbers

```rust
fn main() {
    let pi: f64 = 3.14; // 64-bit floating-point number
    
    println!("Value of Pi: {}", pi);
}
```

In this example, `pi` is a 64-bit floating-point number with a value of 3.14.

## Booleans

Booleans are used to represent true or false values. In Rust, the boolean type is `bool`.

### Example: Booleans

```rust
fn main() {
    let is_snowing: bool = true;
    
    println!("Is it snowing? {}", is_snowing);
}
```

In this example, `is_snowing` is a boolean variable set to `true`.

## Characters

The `char` type represents a single Unicode scalar value. This means it can represent a wide variety of characters from different languages and symbols.

### Example: Characters

```rust
fn main() {
    let letter: char = 'A';
    
    println!("First letter of the alphabet: {}", letter);
}
```

In this example, `letter` is a character variable holding the value 'A'.

## Conclusion

This tutorial covered the basic primitive data types in Rust, including integers, floating-point numbers, Booleans, and characters. Understanding these types is crucial for effectively working with Rust, as they form the foundation of data representation in the language. In subsequent lessons, we will explore more advanced topics and data structures in Rust.
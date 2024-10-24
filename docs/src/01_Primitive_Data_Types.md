# Primitive Data Types in Rust

## Overview

Rust is a statically-typed language, meaning that every variable must have a type known at compile time. Explicitly declaring data types for variables is essential for efficient memory management and ensuring code correctness. Understanding Rust's primitive data types is foundational for writing robust and performant Rust code. This lesson will cover the fundamental primitive data types, including integers, floating-point numbers, Booleans, and characters.

---

## 1. Primitive Data Types in Rust

Rust’s primitive data types, also known as **scalar types**, represent single values. These types are the building blocks for more complex data structures and are crucial for understanding how data is handled in Rust. The four primary scalar types in Rust are:

1. **Integers**
   - **Signed**: `i8`, `i16`, `i32`, `i64`, `i128`
   - **Unsigned**: `u8`, `u16`, `u32`, `u64`, `u128`
2. **Floating-Point Numbers**
   - `f32`
   - `f64`
3. **Boolean**
   - `bool`
4. **Character**
   - `char`

Let's delve into each of these types in detail.

---

### 1.1 Integer Types

Integers are whole numbers without a fractional component. Rust provides both **signed** and **unsigned** integers, each with various bit sizes. The choice between signed and unsigned integers depends on whether you need to represent negative numbers.

#### Signed Integers

Signed integers can store both positive and negative numbers. They are denoted by an `i` followed by the number of bits.

| Type | Range |
|------|-------|
| `i8` | -128 to 127 |
| `i16` | -32,768 to 32,767 |
| `i32` | -2,147,483,648 to 2,147,483,647 |
| `i64` | -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807 |
| `i128` | -170,141,183,460,469,231,731,687,303,715,884,105,728 to 170,141,183,460,469,231,731,687,303,715,884,105,727 |

**Example: Signed Integers**

```rust
fn main() {
    let small_num: i8 = -42;      // Signed 8-bit integer
    let medium_num: i32 = -1_000; // Signed 32-bit integer
    let large_num: i64 = -9_000_000_000; // Signed 64-bit integer
    
    println!("Small number (i8): {}", small_num);
    println!("Medium number (i32): {}", medium_num);
    println!("Large number (i64): {}", large_num);
}
```

**Output:**
```
Small number (i8): -42
Medium number (i32): -1000
Large number (i64): -9000000000
```

#### Unsigned Integers

Unsigned integers can only store positive numbers and zero. They are denoted by a `u` followed by the number of bits.

| Type | Range |
|------|-------|
| `u8` | 0 to 255 |
| `u16` | 0 to 65,535 |
| `u32` | 0 to 4,294,967,295 |
| `u64` | 0 to 18,446,744,073,709,551,615 |
| `u128` | 0 to 340,282,366,920,938,463,463,374,607,431,768,211,455 |

**Example: Unsigned Integers**

```rust
fn main() {
    let small_num: u8 = 200;        // Unsigned 8-bit integer
    let medium_num: u32 = 3_000;    // Unsigned 32-bit integer
    let large_num: u64 = 18_446_744_073_709_551_615; // Unsigned 64-bit integer
    
    println!("Small number (u8): {}", small_num);
    println!("Medium number (u32): {}", medium_num);
    println!("Large number (u64): {}", large_num);
}
```

**Output:**
```
Small number (u8): 200
Medium number (u32): 3000
Large number (u64): 18446744073709551615
```

#### Choosing the Right Integer Type

- **Use smaller types (`i8`, `u8`)** when memory usage is a concern and the range of values is known to be small.
- **Use default types (`i32`, `u32`)** for general-purpose integers.
- **Use larger types (`i64`, `u64`, `i128`, `u128`)** when dealing with very large numbers, such as in cryptographic applications or high-precision calculations.

---

### 1.2 Floating-Point Types

Floating-point numbers represent real numbers with fractional parts. Rust provides two types of floating-point numbers:

- **`f32`**: 32-bit floating-point number.
- **`f64`**: 64-bit floating-point number (default type for floating-point literals).

Floating-point types follow the IEEE 754 standard.

**Example: Floating-Point Types**

```rust
fn main() {
    let pi: f64 = 3.141592653589793;
    let e: f32 = 2.71828;
    
    println!("Value of Pi (f64): {}", pi);
    println!("Value of Euler's number (f32): {}", e);
}
```

**Output:**
```
Value of Pi (f64): 3.141592653589793
Value of Euler's number (f32): 2.71828
```

#### Precision Considerations

- **`f32`**: Provides up to 6 decimal digits of precision.
- **`f64`**: Provides up to 15 decimal digits of precision.

Use `f32` when memory is constrained and high precision is not required. Use `f64` for more precise calculations, which is also the default type for floating-point literals.

**Example: Precision Difference**

```rust
fn main() {
    let precise: f64 = 0.123456789012345;
    let imprecise: f32 = 0.123456789012345_f32;
    
    println!("f64 precision: {}", precise);
    println!("f32 precision: {}", imprecise);
}
```

**Output:**
```
f64 precision: 0.123456789012345
f32 precision: 0.12345679
```

As shown, `f32` loses some precision compared to `f64`.

---

### 1.3 Boolean Type

The Boolean type in Rust is represented by the `bool` keyword. It can take only two values: `true` or `false`. Booleans are commonly used in conditional statements and loops.

**Example: Boolean Type**

```rust
fn main() {
    let is_raining: bool = true;
    let is_sunny: bool = false;
    
    println!("Is it raining? {}", is_raining);
    println!("Is it sunny? {}", is_sunny);
}
```

**Output:**
```
Is it raining? true
Is it sunny? false
```

#### Using Booleans in Control Flow

Booleans are essential for making decisions in your code.

```rust
fn main() {
    let has_license: bool = true;
    
    if has_license {
        println!("You can drive.");
    } else {
        println!("You need a license to drive.");
    }
}
```

**Output:**
```
You can drive.
```

---

### 1.4 Character Type

The `char` type in Rust represents a single Unicode scalar value. Unlike some other languages that limit characters to ASCII, Rust's `char` can represent a wide range of characters, including letters, numbers, and emojis.

**Example: Character Type**

```rust
fn main() {
    let letter: char = 'A';
    let emoji: char = '😊';
    let chinese_char: char = '中';
    
    println!("Letter: {}", letter);
    println!("Emoji: {}", emoji);
    println!("Chinese Character: {}", chinese_char);
}
```

**Output:**
```
Letter: A
Emoji: 😊
Chinese Character: 中
```

#### Unicode Support

Rust's `char` type is 4 bytes in size and can represent any Unicode scalar value. This makes it highly versatile for internationalization and handling diverse character sets.

**Example: Iterating Over a String's Characters**

```rust
fn main() {
    let greeting = "Hello, 世界!";
    
    for c in greeting.chars() {
        println!("{}", c);
    }
}
```

**Output:**
```
H
e
l
l
o
,
 
世
界
!
```

---

## 2. Common Errors and Considerations

Understanding the size and range of each data type is crucial to avoid common programming errors in Rust. Below are some typical issues developers might encounter when working with primitive data types.

### 2.1 Integer Overflow

Each integer type in Rust has a specific range of values it can store. Assigning a value outside this range leads to a compile-time error known as **integer overflow**.

**Example: Integer Overflow**

```rust
fn main() {
    let x: i32 = 2_147_483_648; // This exceeds the i32 range
}
```

**Compiler Error:**
```
error: literal out of range for `i32`
 --> src/main.rs:2:17
  |
2 |     let x: i32 = 2_147_483_648; // This exceeds the i32 range
  |                 ^^^^^^^^^^^^^^^^
```

#### Handling Potential Overflows

Rust does not perform implicit type coercion, which helps prevent overflow errors. However, you can explicitly handle potential overflows using methods like `checked_add`, `wrapping_add`, or `overflowing_add`.

**Example: Checked Addition**

```rust
fn main() {
    let max_i8: i8 = 127;
    match max_i8.checked_add(1) {
        Some(val) => println!("Result: {}", val),
        None => println!("Overflow occurred!"),
    }
}
```

**Output:**
```
Overflow occurred!
```

### 2.2 Type Mismatch

Rust enforces strict type checking. Assigning a value of one type to a variable of another type without explicit conversion leads to a **type mismatch** error.

**Example: Type Mismatch**

```rust
fn main() {
    let y: u64 = -100; // Unsigned type cannot hold a negative value
}
```

**Compiler Error:**
```
error: cannot apply unary operator `-` to type `u64`
 --> src/main.rs:2:17
  |
2 |     let y: u64 = -100; // Unsigned type cannot hold a negative value
  |                 ^^
```

#### Correcting Type Mismatches

Ensure that the value assigned matches the variable's type. Use explicit type casting when necessary.

**Example: Correct Type Assignment**

```rust
fn main() {
    let y: i64 = -100; // Correctly using a signed integer
    let z: u64 = 100;  // Using an unsigned integer for positive value
    
    println!("y: {}", y);
    println!("z: {}", z);
}
```

**Output:**
```
y: -100
z: 100
```

**Example: Explicit Type Casting**

```rust
fn main() {
    let a: i32 = -50;
    let b: u32 = a.abs() as u32; // Convert to unsigned after taking absolute value
    
    println!("a: {}", a);
    println!("b: {}", b);
}
```

**Output:**
```
a: -50
b: 50
```

---

## 3. Summary

Rust’s primitive data types are essential for understanding how to efficiently manage memory and ensure code correctness. By mastering these types, developers can write more reliable and performant Rust code. This lesson covered:

- **Integer Types**: Understanding both signed and unsigned integers and their respective ranges.
- **Floating-Point Types**: Handling real numbers with fractional parts using `f32` and `f64`.
- **Boolean Type**: Utilizing `bool` for logical operations and control flow.
- **Character Type**: Representing single Unicode scalar values with `char`.

### Key Takeaways

- **Memory Management**: Choosing the appropriate data type can optimize memory usage.
- **Type Safety**: Rust’s strict type checking helps prevent bugs related to type mismatches and overflows.
- **Unicode Support**: Rust’s `char` type offers extensive support for a wide range of characters, enhancing internationalization capabilities.

### Next Steps

Further lessons will build on these concepts, delving into **compound data types** such as tuples and arrays, as well as more advanced Rust programming techniques including **ownership**, **borrowing**, and **lifetime annotations**. Mastery of primitive data types lays a solid foundation for exploring Rust’s powerful features and writing efficient, safe, and concurrent code.

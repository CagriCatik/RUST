# Compound Data Types

## Overview

In this tutorial, we will delve into compound data types in the Rust programming language. Compound data types are more complex structures that can store multiple values within a single variable. Rust supports four primary compound data types: arrays, tuples, slices, and strings. Understanding these types is essential for managing collections of data efficiently and for taking advantage of Rust’s memory safety guarantees.

## 1. Compound Data Types in Rust

### 1.1 Arrays

**Arrays** in Rust are fixed-size collections of elements that must all be of the same type, ensuring homogeneity. The size of an array is determined at compile time, and its elements are stored contiguously in memory.

#### Example: Defining an Array

```rust
fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // An array of 5 integers
    
    println!("Number array: {:?}", numbers);
}
```

#### Explanation:
- `numbers` is an array of five 32-bit integers (`i32`).
- Arrays are defined using square brackets. The syntax `[i32; 5]` specifies that the array contains `i32` elements and has a length of 5.
- The `:?` inside the curly braces is a debug format specifier, which allows you to print the entire array.

##### Common Error: Mixed Data Types in Arrays

```rust
fn main() {
    let mix = [1, "apple", true]; // Invalid: mixed types in array
}
```

#### Compiler Error:
```
error: expected integer, found `&str`
```

#### Explanation:
- Rust enforces that all elements in an array must be of the same type. The code above fails because it attempts to mix integers, strings, and booleans within a single array.

### 1.2 Tuples

**Tuples** in Rust can hold multiple values of different types within a single variable. Unlike arrays, tuples can store heterogeneous data. Each element in a tuple can be of a different type, and tuples themselves are of a fixed size, determined at the time of their declaration.

#### Example: Defining a Tuple

```rust
fn main() {
    let human: (&str, i32, bool) = ("Alice", 30, true); // A tuple with a string, an integer, and a boolean
    
    println!("Human tuple: {:?}", human);
}
```

#### Explanation:
- `human` is a tuple that stores a string slice (`&str`), an integer (`i32`), and a boolean (`bool`).
- The tuple elements are accessed by their index, starting from 0. The elements of a tuple are defined within parentheses `()`.

##### Mixed Data Types in Tuples

```rust
fn main() {
    let my_mix = ("Katos", 23, true, [1, 2, 3, 4, 5]); // A tuple with different data types, including an array
    
    println!("Mixed tuple: {:?}", my_mix);
}
```

#### Explanation:
- Tuples can contain different types of data, including other compound data types such as arrays.

### 1.3 Slices

**Slices** in Rust are dynamically sized views into a contiguous sequence of elements within a collection, such as an array. Slices do not own the data they reference, making them useful for borrowing a portion of a collection without copying it.

#### Example: Defining a Slice

```rust
fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; 
    let number_slice: &[i32] = &numbers[1..4]; // A slice of the array from index 1 to 3
    
    println!("Number slice: {:?}", number_slice);
}
```

#### Explanation:
- `number_slice` is a slice of the `numbers` array, containing the elements from index 1 to 3 (`[2, 3, 4]`).
- The syntax `&numbers[1..4]` creates a slice from the `numbers` array, including elements at indices 1, 2, and 3 (but not 4).

### 1.4 Strings and String Slices

**Strings** in Rust are growable, mutable, and owned collections of UTF-8 encoded text. They are stored on the heap and can be modified dynamically. Rust also supports **string slices** (`&str`), which are references to a part of a string or an entire string.

#### Example: String and String Slice

```rust
fn main() {
    let mut greeting: String = String::from("Hello"); // A mutable String
    greeting.push_str(", world!"); // Appending to the string
    
    println!("Greeting: {}", greeting);
    
    let slice: &str = &greeting[0..5]; // A slice of the string
    println!("Slice: {}", slice);
}
```

#### Explanation:
- `greeting` is a `String` object, initially containing `"Hello"`, and is then appended with `", world!"`.
- The `slice` is a string slice that refers to the first five characters of `greeting` (`"Hello"`).
- Strings are stored on the heap, allowing them to grow or shrink at runtime. In contrast, string slices are immutable references to a portion of a string, usually stored on the stack.

#### Memory Allocation and Management

Rust’s memory management is crucial for understanding how strings and slices operate:
- **Strings** are heap-allocated, meaning they are dynamically sized and can be modified.
- **String slices** are references to a part of a string, which do not own the data they refer to. They are used for efficiently accessing parts of a string without copying or owning the data.

#### Common Error: String vs. String Slice

```rust
fn main() {
    let hello = String::from("Hello");
    let slice = &hello[0..5]; // Valid string slice
    
    println!("Slice value: {}", slice);
}
```

- **Strings** can be mutable and are used when ownership of text data is required.
- **String slices** are used to borrow a section of a string without taking ownership, optimizing memory usage.

## 2. Summary

Compound data types in Rust are powerful tools for managing collections of data. This lesson covered:
- **Arrays**: Fixed-size, homogeneous collections.
- **Tuples**: Heterogeneous, fixed-size collections.
- **Slices**: Dynamically-sized views into contiguous sequences.
- **Strings**: Mutable, growable text data stored on the heap.
- **String Slices**: Immutable references to parts of a string.

These data structures are essential for writing efficient and memory-safe code in Rust. Understanding the differences between these types, especially with regard to memory allocation and mutability, is critical for developing robust Rust applications. In future lessons, we will explore more advanced topics, such as Rust’s ownership model, borrowing, and lifetimes.
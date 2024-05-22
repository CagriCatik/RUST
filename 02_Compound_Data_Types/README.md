# Compound Data Types

## Introduction

In Rust, compound data types allow you to group multiple values into a single type. This lesson will cover the following compound data types: arrays, tuples, slices, and strings. Each of these types has its own characteristics and uses, which will be explained in detail.

## Arrays

### Definition and Declaration

Arrays in Rust are collections of elements of the same type. They are of fixed size, meaning their length is known at compile time and cannot be changed. Arrays are defined using square brackets `[]`.

### Example

```rust
fn main() {
    // Declare an array of five integers
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number array: {:?}", numbers);
}
```

In this example:
- `numbers` is an array of type `i32` with a length of 5.
- The `:?` inside the curly braces `{}` is a placeholder for the debug format, which will be explained later.

### Type and Size

Rust requires that the type and size of the array be specified. This can be done as follows:
- `i32` is the type of the elements.
- `5` is the number of elements in the array.

### Common Errors

If you try to print an array without specifying the debug format, you will encounter an error:

```rust
println!("Number array: {}", numbers);  // This will cause an error
```

The correct way to print an array is using the debug format:

```rust
println!("Number array: {:?}", numbers);
```

### Homogeneous Data Types

Arrays in Rust must contain elements of the same type. Trying to mix types will result in a compilation error:

```rust
let mix = [1, 2, "apple", true];  // This will cause an error
```

Rust enforces type safety by ensuring that all elements in an array are of the same type.

## Tuples

### Definition and Declaration

Tuples in Rust can hold multiple values of different types. They have a fixed size and are defined using parentheses `()`.

### Example

```rust
fn main() {
    // Declare a tuple with different types
    let human: (&str, i32, bool) = ("Alice", 30, false);
    println!("Human tuple: {:?}", human);
}
```

In this example:
- `human` is a tuple containing a string slice `&str`, an integer `i32`, and a boolean `bool`.

### Accessing Tuple Elements

You can access the elements of a tuple using dot notation and an index:

```rust
println!("Name: {}", human.0);
println!("Age: {}", human.1);
println!("Is student: {}", human.2);
```

### Mixed Types

Tuples allow mixing different types, making them suitable for grouping related data of various types:

```rust
let mixed_tuple = ("Katos", 23, true, [1, 2, 3, 4, 5]);
println!("Mixed tuple: {:?}", mixed_tuple);
```

## Slices

### Definition and Declaration

Slices are dynamically sized views into a contiguous sequence of elements in an array. They do not have ownership of the data and are used to refer to a portion of an array or a string.

### Example

```rust
fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let number_slice: &[i32] = &numbers;
    println!("Number slice: {:?}", number_slice);
}
```

In this example:
- `number_slice` is a slice of the `numbers` array.
- Slices are defined using an ampersand `&` followed by the array.

### Contiguous Memory

Slices are beneficial because they provide a view into a contiguous sequence of elements, which improves memory efficiency.

### String Slices

String slices are references to portions of strings. They are immutable and do not own the data they reference:

```rust
fn main() {
    let s = String::from("Hello, world!");
    let slice = &s[0..5];
    println!("Slice: {}", slice);
}
```

In this example:
- `slice` is a string slice referring to the first five characters of the string `s`.

## Strings

### Definition and Declaration

Strings in Rust are growable, mutable, and owned data types. They are stored on the heap, allowing them to change size dynamically.

### Example

```rust
fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("String: {}", s);
}
```

In this example:
- `s` is a mutable string (`String`).
- `push_str` method appends `", world!"` to the string.

### Differences Between Strings and String Slices

- **Strings (`String`)** are mutable and stored on the heap, allowing dynamic resizing.
- **String slices (`&str`)** are immutable references to a portion of a string, stored on the stack, and used for efficient memory access.

## Memory Management

Rust's ownership system ensures memory safety without a garbage collector. Understanding how Rust manages memory with stack and heap is crucial for efficient programming.

### Heap vs. Stack

- **Stack** is fast but has a limited size. It stores data of fixed size, like integer and boolean values.
- **Heap** is slower but can grow dynamically. It stores data like strings and vectors that can change size.

## Conclusion

This lesson covered the basics of compound data types in Rust, including arrays, tuples, slices, and strings. Understanding these types and their memory management characteristics is essential for writing efficient and safe Rust programs. Future lessons will delve deeper into more advanced topics such as ownership, borrowing, and memory allocation in Rust.
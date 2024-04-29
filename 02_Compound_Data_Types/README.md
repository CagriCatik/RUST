# Compound Data Types

In this lesson, we'll delve into compound data types in Rust. Compound data types are essential for organizing and manipulating data in Rust programs. We'll cover arrays, tuples, slices, and strings in detail.

## Arrays

Arrays in Rust are fixed-size collections of elements of the same type. They are homogeneous, meaning all elements must be of the same type.

```rust
fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number array: {:?}", numbers);
}
```

Explanation:

- `numbers`: Declares an array variable.
- `: [i32; 5]`: Specifies the data type (i32) and size (5) of the array.
- `[1, 2, 3, 4, 5]`: Initializes the array with values.
- `{:?}`: Uses debug format to print the array.

## Error Handling

When printing arrays, ensure to use the correct formatting. Rust offers two formats: debuggable and display.

```rust
println!("Number array: {:?}", numbers); // Use debug format
```

## Tuples

Tuples are heterogeneous collections of elements of fixed size. Unlike arrays, tuples can contain elements of different types.

```rust
fn main() {
    let human: (String, i32, bool) = (String::from("Alice"), 30, false);
    println!("Human tuple: {:?}", human);
}
```

Explanation:

- `human`: Declares a tuple variable.
- `(String, i32, bool)`: Specifies the data types of elements in the tuple.
- `(String::from("Alice"), 30, false)`: Initializes the tuple with values.

## Mutable Tuple

Tuples can also be mutable.

```rust
fn main() {
    let mut human: (String, i32, bool) = (String::from("Alice"), 30, false);
    human.1 = 31; // Modify the second element
    println!("Human tuple: {:?}", human);
}
```

## Slices

Slices provide a dynamically sized view into a contiguous sequence of elements. They are references to arrays or other data structures.

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let number_slice = &numbers[..];
    println!("Number slice: {:?}", number_slice);
}
```

Explanation:

- `&numbers[..]`: Creates a slice referencing the entire array.
- `{:?}`: Uses debug format to print the slice.

## Strings

Strings in Rust are growable and mutable. They are owned types allocated on the heap.

```rust
fn main() {
    let mut greeting = String::from("Hello");
    greeting.push_str(", world!");
    println!("{}", greeting);
}
```

Explanation:

- `String::from("Hello")`: Initializes a string.
- `push_str()`: Appends a string slice to the string.

## String Slices

String slices are references to portions of strings. They are immutable and stored on the stack.

```rust
fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5]; // String slice referencing "hello"
    println!("{}", hello);
}
```

Explanation:

- `&s[0..5]`: Creates a string slice referencing the first five characters.

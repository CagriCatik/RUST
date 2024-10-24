#  Compound Data Types in Rust

## Overview

While primitive data types handle single values, compound data types allow you to store multiple values within a single variable. Rust provides four primary compound data types: **arrays**, **tuples**, **slices**, and **strings**. Mastering these types is essential for managing collections of data efficiently and leveraging Rust’s powerful memory safety guarantees. This lesson will explore each of these compound data types in detail, complete with examples, explanations, and common pitfalls to watch out for.

---

## 1. Compound Data Types in Rust

Rust’s compound data types, also known as **aggregate types**, enable the storage of multiple values within a single variable. These types are indispensable for handling more complex data structures and algorithms. The four primary compound types in Rust are:

1. **Arrays**
2. **Tuples**
3. **Slices**
4. **Strings and String Slices**

Let’s explore each of these in detail.

---

### 1.1 Arrays

**Arrays** in Rust are fixed-size collections of elements that must all be of the same type, ensuring homogeneity. The size of an array is determined at compile time, and its elements are stored contiguously in memory, which allows for efficient access and manipulation.

#### Example: Defining an Array

```rust
fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // An array of 5 integers
    
    println!("Number array: {:?}", numbers);
}
```

**Output:**
```
Number array: [1, 2, 3, 4, 5]
```

#### Explanation:
- `numbers` is an array of five 32-bit integers (`i32`).
- Arrays are defined using square brackets. The syntax `[i32; 5]` specifies that the array contains `i32` elements and has a length of 5.
- The `:?` inside the curly braces is a debug format specifier, which allows you to print the entire array.

#### Accessing Array Elements

You can access elements in an array using indexing, which starts at 0.

```rust
fn main() {
    let numbers: [i32; 5] = [10, 20, 30, 40, 50];
    
    let first = numbers[0];
    let third = numbers[2];
    
    println!("First number: {}", first);
    println!("Third number: {}", third);
}
```

**Output:**
```
First number: 10
Third number: 30
```

#### Initializing Arrays with Repeated Values

Rust allows you to initialize an array where all elements have the same value using a shorthand syntax.

```rust
fn main() {
    let zeros: [u8; 10] = [0; 10]; // An array of ten u8 integers, all initialized to 0
    
    println!("Zeros array: {:?}", zeros);
}
```

**Output:**
```
Zeros array: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
```

#### Common Error: Mixed Data Types in Arrays

```rust
fn main() {
    let mix = [1, "apple", true]; // Invalid: mixed types in array
}
```

**Compiler Error:**
```
error[E0308]: mismatched types
 --> src/main.rs:2:17
  |
2 |     let mix = [1, "apple", true]; // Invalid: mixed types in array
  |                 ^ expected integer, found `&str`
```

#### Explanation:
- Rust enforces that all elements in an array must be of the same type.
- The code above fails because it attempts to mix integers, strings, and booleans within a single array, which violates Rust's type safety.

#### Iterating Over Arrays

You can iterate over arrays using loops to perform operations on each element.

```rust
fn main() {
    let numbers = [2, 4, 6, 8, 10];
    
    for number in numbers.iter() {
        println!("Number: {}", number);
    }
}
```

**Output:**
```
Number: 2
Number: 4
Number: 6
Number: 8
Number: 10
```

---

### 1.2 Tuples

**Tuples** in Rust can hold multiple values of different types within a single variable. Unlike arrays, tuples can store heterogeneous data. Each element in a tuple can be of a different type, and tuples themselves are of a fixed size, determined at the time of their declaration.

#### Example: Defining a Tuple

```rust
fn main() {
    let human: (&str, i32, bool) = ("Alice", 30, true); // A tuple with a string, an integer, and a boolean
    
    println!("Human tuple: {:?}", human);
}
```

**Output:**
```
Human tuple: ("Alice", 30, true)
```

#### Explanation:
- `human` is a tuple that stores a string slice (`&str`), an integer (`i32`), and a boolean (`bool`).
- The tuple elements are accessed by their index, starting from 0. The elements of a tuple are defined within parentheses `()`.

#### Accessing Tuple Elements

You can access individual elements of a tuple using pattern matching or by using dot notation with the index.

**Using Dot Notation:**

```rust
fn main() {
    let person = ("Bob", 25, false);
    
    let name = person.0;
    let age = person.1;
    let is_employed = person.2;
    
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Is Employed: {}", is_employed);
}
```

**Output:**
```
Name: Bob
Age: 25
Is Employed: false
```

**Using Pattern Matching:**

```rust
fn main() {
    let person = ("Carol", 28, true);
    
    let (name, age, is_employed) = person;
    
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Is Employed: {}", is_employed);
}
```

**Output:**
```
Name: Carol
Age: 28
Is Employed: true
```

#### Common Use Cases for Tuples

- **Returning Multiple Values:** Functions can return multiple values bundled in a tuple.
  
  ```rust
  fn get_person() -> (&'static str, u8) {
      ("Dave", 40)
  }
  
  fn main() {
      let (name, age) = get_person();
      println!("Name: {}, Age: {}", name, age);
  }
  ```
  
  **Output:**
  ```
  Name: Dave, Age: 40
  ```

- **Grouping Related Data:** Tuples can group related but different types of data without needing to define a struct.

#### Mixed Data Types in Tuples

```rust
fn main() {
    let my_mix = ("Katos", 23, true, [1, 2, 3, 4, 5]); // A tuple with different data types, including an array
    
    println!("Mixed tuple: {:?}", my_mix);
}
```

**Output:**
``    Mixed tuple: ("Katos", 23, true, [1, 2, 3, 4, 5])
```

#### Explanation:
- Tuples can contain different types of data, including other compound data types such as arrays, enhancing their flexibility.

#### Destructuring Tuples

Destructuring allows you to break a tuple into its individual components for easier access and manipulation.

```rust
fn main() {
    let coordinates = (10, 20, 30);
    
    let (x, y, z) = coordinates;
    
    println!("x: {}, y: {}, z: {}", x, y, z);
}
```

**Output:**
```
x: 10, y: 20, z: 30
```

---

### 1.3 Slices

**Slices** in Rust are dynamically sized views into a contiguous sequence of elements within a collection, such as an array or a `String`. Slices do not own the data they reference, making them useful for borrowing a portion of a collection without copying it. They provide a way to reference a segment of a collection without taking ownership, thereby adhering to Rust’s ownership and borrowing rules.

#### Example: Defining a Slice

```rust
fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; 
    let number_slice: &[i32] = &numbers[1..4]; // A slice of the array from index 1 to 3
    
    println!("Number slice: {:?}", number_slice);
}
```

**Output:**
```
Number slice: [2, 3, 4]
```

#### Explanation:
- `number_slice` is a slice of the `numbers` array, containing the elements from index 1 to 3 (`[2, 3, 4]`).
- The syntax `&numbers[1..4]` creates a slice from the `numbers` array, including elements at indices 1, 2, and 3 (but not 4).

#### Slices with Strings

Slices are also commonly used with `String` types to reference a portion of a string.

```rust
fn main() {
    let greeting = String::from("Hello, Rust!");
    let hello = &greeting[0..5]; // Slicing the first five characters
    
    println!("Greeting: {}", greeting);
    println!("Slice: {}", hello);
}
```

**Output:**
``    Greeting: Hello, Rust!
    Slice: Hello
```

#### Mutable Slices

Slices can also be mutable, allowing you to modify the referenced data.

```rust
fn main() {
    let mut numbers = [1, 2, 3, 4, 5];
    let number_slice: &mut [i32] = &mut numbers[2..5];
    
    number_slice[0] = 30;
    number_slice[2] = 50;
    
    println!("Modified numbers: {:?}", numbers);
}
```

**Output:**
``    Modified numbers: [1, 2, 30, 4, 50]
```

#### Common Error: Out-of-Bounds Slicing

```rust
fn main() {
    let numbers = [10, 20, 30, 40, 50];
    let invalid_slice = &numbers[3..6]; // Index 6 is out of bounds
}
```

**Compiler Error:**
```
error: index 6 out of bounds for array of length 5
 --> src/main.rs:3:25
  |
3 |     let invalid_slice = &numbers[3..6]; // Index 6 is out of bounds
  |                         ^^^^^^^^
```

#### Explanation:
- Attempting to create a slice that exceeds the bounds of the array will result in a compile-time error, ensuring memory safety.

#### Iterating Over Slices

You can iterate over slices similarly to how you iterate over arrays.

```rust
fn main() {
    let fruits = ["apple", "banana", "cherry", "date", "elderberry"];
    let fruit_slice = &fruits[1..4];
    
    for fruit in fruit_slice.iter() {
        println!("Fruit: {}", fruit);
    }
}
```

**Output:**
``    Fruit: banana
    Fruit: cherry
    Fruit: date
```

#### Using Slices in Functions

Slices are often used as function parameters to allow functions to operate on subsets of data without taking ownership.

```rust
fn print_slice(slice: &[i32]) {
    for number in slice {
        println!("{}", number);
    }
}

fn main() {
    let numbers = [100, 200, 300, 400, 500];
    let middle = &numbers[1..4];
    
    print_slice(middle);
}
```

**Output:**
``    200
    300
    400
```

---

### 1.4 Strings and String Slices

**Strings** in Rust are growable, mutable, and owned collections of UTF-8 encoded text. They are stored on the heap and can be modified dynamically. Rust also supports **string slices** (`&str`), which are references to a part of a string or an entire string. Understanding the distinction between `String` and `&str` is crucial for effective string manipulation and memory management in Rust.

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

**Output:**
``    Greeting: Hello, world!
    Slice: Hello
```

#### Explanation:
- `greeting` is a `String` object, initially containing `"Hello"`, and is then appended with `", world!"`.
- The `slice` is a string slice that refers to the first five characters of `greeting` (`"Hello"`).
- Strings are stored on the heap, allowing them to grow or shrink at runtime. In contrast, string slices are immutable references to a portion of a string, usually stored on the stack.

#### Creating Strings

You can create strings in Rust using several methods:

- **Using `String::new`:** Creates an empty `String`.
  
  ```rust
  fn main() {
      let mut empty = String::new();
      empty.push_str("Hello, Rust!");
      
      println!("Empty string: {}", empty);
  }
  ```
  
  **Output:**
  ```
    Empty string: Hello, Rust!
  ```

- **Using `String::from`:** Converts a string literal into a `String`.
  
  ```rust
  fn main() {
      let hello = String::from("Hello");
      println!("Hello string: {}", hello);
  }
  ```
  
  **Output:**
  ```
    Hello string: Hello
  ```

- **Using the `to_string` Method:** Converts any type that implements the `Display` trait into a `String`.
  
  ```rust
  fn main() {
      let number = 42;
      let number_str = number.to_string();
      
      println!("Number as string: {}", number_str);
  }
  ```
  
  **Output:**
  ```
    Number as string: 42
  ```

#### Modifying Strings

Strings in Rust are mutable, allowing for dynamic modifications such as appending, inserting, or removing characters.

- **Appending to a String:**
  
  ```rust
  fn main() {
      let mut message = String::from("Hello");
      message.push(' ');
      message.push_str("Rust!");
      
      println!("Message: {}", message);
  }
  ```
  
  **Output:**
  ```
    Message: Hello Rust!
  ```

- **Inserting into a String:**
  
  ```rust
  fn main() {
      let mut message = String::from("Hello Rust!");
      message.insert(5, ','); // Inserts a comma after "Hello"
      
      println!("Message: {}", message);
  }
  ```
  
  **Output:**
  ```
    Message: Hello, Rust!
  ```

- **Removing from a String:**
  
  ```rust
  fn main() {
      let mut message = String::from("Hello, Rust!");
      message.pop(); // Removes the last character '!'
      
      println!("Message after pop: {}", message);
  }
  ```
  
  **Output:**
  ```
    Message after pop: Hello, Rust
  ```

#### Memory Allocation and Management

Understanding how Rust manages memory for `String` and string slices is crucial for writing efficient and safe code.

- **Strings (`String`):**
  - **Heap-Allocated:** Stored on the heap, allowing for dynamic sizing.
  - **Ownership:** `String` owns the data it contains, and when it goes out of scope, the memory is freed automatically.
  - **Mutability:** Can be mutated to grow or shrink as needed.

- **String Slices (`&str`):**
  - **Reference:** A `&str` is a reference to a string, which can be either a string literal or a portion of a `String`.
  - **Immutability:** By default, string slices are immutable, meaning the data they point to cannot be changed through the slice.
  - **Efficiency:** Useful for borrowing data without taking ownership, avoiding unnecessary data copying.

#### Common Error: String vs. String Slice

```rust
fn main() {
    let hello = String::from("Hello");
    let slice = &hello[0..5]; // Valid string slice
    
    println!("Slice value: {}", slice);
}
```

**Explanation:**
- **Strings** can be mutable and are used when ownership of text data is required.
- **String slices** are used to borrow a section of a string without taking ownership, optimizing memory usage.

#### Borrowing and Lifetimes with Slices

When working with slices, it’s important to understand Rust’s borrowing and lifetime rules to ensure memory safety.

```rust
fn main() {
    let full_string = String::from("Hello, Rust!");
    let part = &full_string[7..12];
    
    println!("Full string: {}", full_string);
    println!("Part: {}", part);
}
```

**Output:**
``    Full string: Hello, Rust!
    Part: Rust
```

**Explanation:**
- `part` is a string slice that borrows a portion of `full_string`.
- As long as `part` is in use, `full_string` cannot be modified in a way that would invalidate the slice.

#### Working with Unicode and String Slices

Rust’s strings are UTF-8 encoded, which allows them to handle a wide range of characters, including those outside the ASCII range.

```rust
fn main() {
    let greeting = String::from("こんにちは"); // "Hello" in Japanese
    let slice = &greeting[0..3]; // Slicing part of a multi-byte character
    
    println!("Greeting: {}", greeting);
    println!("Slice: {}", slice);
}
```

**Output:**
``    Greeting: こんにちは
    Slice: こんに
```

**Explanation:**
- Each Japanese character in `"こんにちは"` takes up 3 bytes.
- Slicing must respect character boundaries to avoid invalid UTF-8 sequences.

#### Common Error: Invalid UTF-8 Slicing

```rust
fn main() {
    let greeting = String::from("Hello, 世界");
    let invalid_slice = &greeting[0..7]; // Potentially splits a multi-byte character
    
    println!("Slice: {}", invalid_slice);
}
```

**Compiler Error:**
```
error: byte index 7 is not a char boundary; it is inside '世' (bytes 7..10) of `Hello, 世界`
 --> src/main.rs:3:24
  |
3 |     let invalid_slice = &greeting[0..7]; // Potentially splits a multi-byte character
  |                        ^^^^^
```

#### Explanation:
- Rust enforces that string slices must align with character boundaries to maintain valid UTF-8 encoding.
- Attempting to slice in the middle of a multi-byte character results in a compile-time error.

---

## 2. Common Errors and Considerations

Understanding the intricacies of Rust's compound data types is crucial to avoid common programming errors. Below are some typical issues developers might encounter when working with arrays, tuples, slices, and strings.

### 2.1 Array Index Out of Bounds

Accessing elements outside the valid range of an array will result in a compile-time error, ensuring memory safety.

```rust
fn main() {
    let numbers = [10, 20, 30, 40, 50];
    let out_of_bounds = numbers[5]; // Index 5 does not exist in a 5-element array
}
```

**Compiler Error:**
```
error: index out of bounds: the length is 5 but the index is 5
 --> src/main.rs:3:24
  |
3 |     let out_of_bounds = numbers[5]; // Index 5 does not exist in a 5-element array
  |                        ^^^^^
```

#### Explanation:
- Rust checks array bounds at compile time when possible, and at runtime otherwise.
- Attempting to access an index outside the array's bounds will result in a compile-time or runtime error, preventing undefined behavior.

### 2.2 Tuple Mismatch in Pattern Matching

When destructuring tuples, the number of variables must match the number of elements in the tuple.

```rust
fn main() {
    let person = ("Eve", 29);
    
    let (name, age, is_employed) = person; // Too many variables
}
```

**Compiler Error:**
```
error[E0308]: pattern requires 3 elements but tuple has 2
 --> src/main.rs:4:9
  |
4 |     let (name, age, is_employed) = person; // Too many variables
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 elements
```

#### Explanation:
- The tuple `person` has two elements, but the destructuring pattern expects three.
- Ensuring that the number of variables matches the tuple's size prevents such mismatches.

### 2.3 Invalid Slice Ranges

Creating slices with invalid ranges, such as overlapping multi-byte characters or out-of-bounds indices, will result in compile-time errors.

```rust
fn main() {
    let text = String::from("Hello, 世界");
    let invalid_slice = &text[7..9]; // Splits the multi-byte character '世'
}
```

**Compiler Error:**
```
error: byte index 9 is not a char boundary; it is inside '世' (bytes 7..10) of `Hello, 世界`
 --> src/main.rs:3:26
  |
3 |     let invalid_slice = &text[7..9]; // Splits the multi-byte character '世'
  |                          ^^^^^
```

#### Explanation:
- Rust enforces that slices must respect character boundaries to maintain valid UTF-8 encoding.
- Attempting to slice within a multi-byte character leads to an error, ensuring string integrity.

### 2.4 String Ownership and Borrowing Issues

Mismanaging ownership and borrowing with `String` and `&str` can lead to compilation errors.

```rust
fn main() {
    let greeting = String::from("Hello, Rust!");
    let slice = &greeting[..];
    
    drop(greeting); // Explicitly dropping greeting
    
    println!("Slice: {}", slice); // Error: greeting was moved
}
```

**Compiler Error:**
```
error[E0382]: borrow of moved value: `greeting`
 --> src/main.rs:6:29
  |
2 |     let greeting = String::from("Hello, Rust!");
  |         -------- move occurs because `greeting` has type `String`, which does not implement the `Copy` trait
3 |     let slice = &greeting[..];
  |                  -------- value borrowed here
4 |     
5 |     drop(greeting); // Explicitly dropping greeting
  |          -------- value moved here
6 |     println!("Slice: {}", slice); // Error: greeting was moved
  |                             ^^^^^ value borrowed here after move
```

#### Explanation:
- After calling `drop(greeting)`, `greeting` is moved and no longer valid.
- However, `slice` is a reference to `greeting`, leading to a conflict as `greeting` is no longer available.
- Properly managing lifetimes and ensuring that references do not outlive the data they point to is essential.

### 2.5 Immutable vs. Mutable Slices

Attempting to mutate data through an immutable slice will result in a compiler error.

```rust
fn main() {
    let mut numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4]; // Immutable slice
    
    slice[0] = 20; // Error: cannot assign to immutable index
}
```

**Compiler Error:**
```
error[E0594]: cannot assign to `slice[0]` which is behind a `&` reference
 --> src/main.rs:4:5
  |
4 |     slice[0] = 20; // Error: cannot assign to immutable index
  |     ^^^^^^^^^^^^^^ cannot assign
```

#### Explanation:
- The slice `slice` is immutable, meaning you cannot modify its elements.
- To modify elements through a slice, you must create a mutable slice using `&mut`.

**Correct Usage with Mutable Slice:**

```rust
fn main() {
    let mut numbers = [1, 2, 3, 4, 5];
    let slice = &mut numbers[1..4]; // Mutable slice
    
    slice[0] = 20; // Now valid
    
    println!("Modified numbers: {:?}", numbers);
}
```

**Output:**
```    Modified numbers: [1, 20, 3, 4, 5]
```

---

## 3. Summary

Compound data types in Rust are powerful tools for managing collections of data efficiently and safely. This lesson covered:

- **Arrays**: Fixed-size, homogeneous collections. Ideal for storing elements of the same type with a known size at compile time.
- **Tuples**: Heterogeneous, fixed-size collections. Useful for grouping related but different types of data.
- **Slices**: Dynamically-sized views into contiguous sequences. Enable borrowing parts of collections without taking ownership.
- **Strings and String Slices**: `String` for owned, mutable, and growable text data stored on the heap, and `&str` for immutable references to string data.

### Key Takeaways

- **Memory Efficiency**: Choosing the appropriate compound data type can optimize memory usage and performance.
- **Type Safety**: Rust’s strict type checking ensures that operations on compound types are safe and prevent common bugs.
- **Ownership and Borrowing**: Understanding how ownership and borrowing work with compound types is crucial for writing safe and efficient Rust code.
- **Flexibility with Slices**: Slices provide a flexible way to reference portions of data without the overhead of copying, adhering to Rust’s zero-cost abstractions philosophy.

### Next Steps

Building upon your understanding of primitive and compound data types, future lessons will explore more advanced Rust concepts, including:

- **Ownership Model**: Deep dive into Rust’s ownership rules, enabling memory safety without a garbage collector.
- **Borrowing and Lifetimes**: Learn how to manage references and ensure data validity through lifetimes.
- **Advanced Data Structures**: Explore collections like vectors, hash maps, and custom data structures.
- **Concurrency**: Harness Rust’s concurrency features to write safe and efficient multi-threaded programs.

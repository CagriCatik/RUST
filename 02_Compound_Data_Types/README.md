# Compound Data Types

In this lesson, we will explore the compound data types in Rust: arrays, tuples, slices, and strings. Each of these types allows for the storage and manipulation of multiple values in a structured way. We will examine the characteristics, usage, and differences between these types, ensuring a comprehensive understanding of each.

## Arrays

Arrays in Rust are a fixed-size collection of elements of the same type. This means that once an array is declared, its size cannot be changed, and all elements within the array must be of the same type.

### Declaration and Initialization

Here is an example of how to declare and initialize an array in Rust:

```rust
fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number array: {:?}", numbers);
}
```

In this code:
- `let numbers: [i32; 5]` declares an array named `numbers` of type `i32` with a length of 5.
- The array is initialized with the values `[1, 2, 3, 4, 5]`.
- `println!("Number array: {:?}", numbers);` prints the array using the debug format specifier `:?`.

### Error Handling

If you attempt to mix data types in an array, Rust will generate a compilation error:

```rust
fn main() {
    let mix = [1, "Apple", true]; // This line will cause a compilation error
    println!("Mix array: {:?}", mix);
}
```

The above code will result in the following error:
```
error[E0308]: mismatched types
expected integer, found `&str`
```

This error occurs because Rust arrays must be homogeneous, containing elements of the same type.

### Accessing Array Elements

You can access elements of an array using indices:

```rust
fn main() {
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("First fruit: {}", fruits[0]);
    println!("Second fruit: {}", fruits[1]);
    println!("Third fruit: {}", fruits[2]);
    println!("Fruit array: {:?}", fruits);
}
```

## Tuples

Tuples in Rust are used to group multiple values of different types into a single compound type. Tuples have a fixed size and can contain heterogeneous types.

### Declaration and Initialization

Here is an example of declaring and initializing a tuple:

```rust
fn main() {
    let person: (&str, i32, bool) = ("Alice", 30, false);
    println!("Person tuple: {:?}", person);
}
```

In this code:
- `let person: (&str, i32, bool)` declares a tuple named `person` containing a `&str`, an `i32`, and a `bool`.
- The tuple is initialized with the values `("Alice", 30, false)`.
- `println!("Person tuple: {:?}", person);` prints the tuple using the debug format specifier `:?`.

### Accessing Tuple Elements

You can access elements of a tuple using pattern matching or indexing:

```rust
fn main() {
    let person = ("Alice", 30, false);
    let (name, age, is_student) = person;
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Is student: {}", is_student);
    println!("Person tuple: {:?}", person);
}
```

You can also access elements by index:

```rust
fn main() {
    let person = ("Alice", 30, false);
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is student: {}", person.2);
}
```

## Slices

Slices in Rust are dynamically sized views into a contiguous sequence of elements. They do not own the data they reference and can be used to borrow a section of an array.

### Declaration and Initialization

Here is an example of declaring and using a slice:

```rust
fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let number_slice: &[i32] = &numbers[1..4];
    println!("Number slice: {:?}", number_slice);
}
```

In this code:
- `let number_slice: &[i32] = &numbers[1..4];` creates a slice that references elements from index 1 to 3 of the `numbers` array.
- `println!("Number slice: {:?}", number_slice);` prints the slice.

## Strings and String Slices

Strings in Rust are growable, mutable, and stored on the heap. They are owned types, meaning they manage their memory.

### String Declaration and Initialization

Here is an example of declaring and using a `String`:

```rust
fn main() {
    let mut greeting = String::from("Hello");
    greeting.push_str(", world!");
    println!("{}", greeting);
}
```

In this code:
- `let mut greeting = String::from("Hello");` creates a mutable `String` initialized with `"Hello"`.
- `greeting.push_str(", world!");` appends `", world!"` to the `greeting` string.
- `println!("{}", greeting);` prints the resulting string.

### String Slices

String slices are references to a portion of a string. They are immutable and usually used for borrowing.

Here is an example:

```rust
fn main() {
    let greeting = String::from("Hello, world!");
    let hello = &greeting[0..5];
    println!("Slice: {}", hello);
}
```

In this code:
- `let hello = &greeting[0..5];` creates a string slice referencing the first five characters of `greeting`.
- `println!("Slice: {}", hello);` prints the slice.

### Differences between `String` and `&str`

- `String` is a growable, mutable, owned type stored on the heap.
- `&str` is an immutable reference to a string slice, usually stored on the stack.

Here is an example demonstrating these differences:

```rust
fn main() {
    let string = String::from("Hello, world!");
    let string_slice: &str = &string;
    println!("String: {}", string);
    println!("String slice: {}", string_slice);
}
```

In summary, understanding compound data types in Rust is crucial for efficient memory management and effective programming. Arrays, tuples, slices, and strings each have unique characteristics and uses, allowing for flexible and powerful data handling in Rust applications.
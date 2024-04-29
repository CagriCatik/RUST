
# Ownership

In this lesson, we're diving deep into ownership in Rust. Understanding ownership is crucial in Rust programming as it ensures memory safety without the need for a garbage collector. Let's explore why ownership exists, the problems it solves, and how Rust implements it.

## Why Ownership?

Many programming languages, like C and C++, allow manual control over memory allocation and deallocation. While this flexibility can be powerful, it often leads to bugs like double frees or memory leaks. To address these issues, some languages introduce garbage collection. However, garbage collection can introduce runtime overhead and pauses, making it unsuitable for performance-critical applications.

Rust takes a different approach with ownership, ensuring memory safety at compile time without sacrificing performance. This concept, recommended by experts and even endorsed by the White House for cybersecurity, is a cornerstone of Rust's design.

## Ownership Rules

Before we dive into code examples, let's understand the three fundamental ownership rules in Rust:

1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

Violating these rules results in compiler errors, ensuring that memory-related bugs are caught early in the development process.

Now, let's explore these rules through code examples.

## Example 1: Single Owner

```rust
fn main() {
    let s1 = String::from("Rust");

    let length = calculate_length(&s1); // Pass a reference to s1

    println!("The length of '{}' is {}", s1, length);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

In this example, `s1` is the owner of the `String` value "Rust". We pass a reference `&s1` to the `calculate_length` function, which calculates the length of the string without taking ownership. The ownership of the string remains with `s1`.

## Example 2: Transferring Ownership

```rust
fn main() {
    let s1 = String::from("Rust");
    let s2 = s1; // Ownership transferred to s2

    println!("{}", s1); // Error: s1 no longer owns the string
    println!("{}", s2); // Prints "Rust"
}
```

Here, ownership of the string is transferred from `s1` to `s2` by assigning `s1` to `s2`. As a result, `s1` can no longer be used to access the string.

## Example 3: Ownership and Scope

```rust
fn main() {
    let s1 = String::from("Rust");

    {
        let length = calculate_length(&s1);
        println!("The length of '{}' is {}", s1, length);
    } // s1 goes out of scope here and is dropped

    // println!("{}", s1); // Error: s1 is out of scope
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

In this example, `s1` goes out of scope after the inner block, and the string is dropped automatically. Attempting to use `s1` outside its scope results in a compiler error.

## Conclusion

Ownership is a powerful concept in Rust that ensures memory safety without sacrificing performance. By following the ownership rules, you can write safer and more reliable code. In the next lesson, we'll explore borrowing and references, essential concepts for working with ownership in Rust. Stay tuned!

# Shadowing

## Overview

In this lesson, we will explore the concept of **shadowing** in Rust. Shadowing allows you to declare a new variable with the same name as a previous variable, effectively replacing the old variable within a certain scope. This is different from simply making a variable mutable, as it allows for more flexibility, including the ability to change the variable's type. Understanding shadowing is essential for writing clear and idiomatic Rust code.

## 1. What is Shadowing?

### 1.1 Definition

Shadowing in Rust refers to the ability to declare a new variable with the same name as a previously declared variable. When a new variable is declared with the same name, it "shadows" the previous one, meaning the previous variable is no longer accessible in that scope, and the new variable takes precedence.

### 1.2 How Shadowing Works

When you declare a new variable with the same name as an existing one, Rust allows the new variable to shadow the old one. This means that the new variable takes over, and the previous variable is effectively hidden within the scope where the shadowing occurs.

#### Example: Basic Shadowing

```rust
fn main() {
    let x = 5; // First declaration of `x`
    let x = x + 1; // Shadowing the first `x` with a new `x`
    
    println!("The value of x is: {}", x); // Prints 6
}
```

#### Explanation:
- The first `x` is initialized with the value `5`.
- The second `x` shadows the first `x`, adding `1` to its value, resulting in `6`.
- When `x` is printed, it displays the value `6`, which corresponds to the shadowed `x`.

## 2. Shadowing vs. Mutability

### 2.1 Shadowing is Not Mutability

It's important to understand that shadowing is different from marking a variable as mutable. While both allow a variable's value to change, shadowing does so by creating a new variable entirely, while mutability allows for in-place modification of an existing variable.

#### Example: Attempt to Modify Without Mutability

```rust
fn main() {
    let x = 5;
    // x = 10; // ERROR: Cannot assign twice to immutable variable `x`
}
```

#### Explanation:
- Attempting to reassign `x` without marking it as mutable results in a compilation error.
- Shadowing avoids this by creating a new variable, which is not the same as modifying the original.

### 2.2 Benefits of Shadowing

Shadowing allows you to reuse variable names without needing to mark them as mutable, and it can even allow you to change the type of a variable while reusing its name.

#### Example: Changing Type with Shadowing

```rust
fn main() {
    let spaces = "   "; // `spaces` is a string slice
    let spaces = spaces.len(); // `spaces` is now an integer
    
    println!("The number of spaces is: {}", spaces);
}
```

#### Explanation:
- The first `spaces` variable is a string slice containing spaces.
- The second `spaces` variable shadows the first and stores the length of the string, changing its type to an integer.
- This is a powerful use of shadowing, allowing you to reuse the same name while changing the data it holds.

## 3. Shadowing in Different Scopes

Shadowing can occur in different scopes, such as within nested blocks. When a variable is shadowed within a block, the original variable is still accessible outside that block.

#### Example: Shadowing in Nested Scopes

```rust
fn main() {
    let x = 5;
    
    {
        let x = x * 2; // Shadows `x` within this block
        println!("The value of x in the inner scope is: {}", x); // Prints 10
    }
    
    println!("The value of x in the main scope is: {}", x); // Prints 5
}
```

#### Explanation:
- The variable `x` is declared with the value `5` in the main scope.
- Inside the nested block, a new `x` shadows the original `x`, doubling its value to `10`.
- Outside the block, the original `x` remains unaffected and retains its value of `5`.

## 4. Shadowing and Type Changes

One of the unique features of shadowing is the ability to change the type of a variable while reusing the same name. This allows for more flexible and concise code, especially when dealing with different stages of data transformation.

#### Example: Shadowing to Change Type

```rust
fn main() {
    let guess = "42"; // `guess` is a string slice
    
    let guess: i32 = guess.trim().parse().expect("Not a number!"); // `guess` is now an i32
    
    println!("The guess is: {}", guess);
}
```

#### Explanation:
- The initial `guess` is a string slice containing the text `"42"`.
- The second `guess` shadows the first and parses the string into an integer (`i32`).
- This allows for seamless type conversion while maintaining the same variable name.

## 5. Common Pitfalls and Error Handling

### 5.1 Reassigning Without Shadowing or Mutability

If you attempt to reassign a value to a variable without using `let` for shadowing or `mut` for mutability, Rust will produce a compile-time error.

#### Example: Error in Reassignment

```rust
fn main() {
    let x = 5;
    x = 10; // ERROR: Cannot assign twice to immutable variable `x`
}
```

### 5.2 Mutability vs. Shadowing

Shadowing creates a new variable, while mutability modifies the existing one. Shadowing allows you to declare a new variable with the same name, effectively creating a fresh variable that can even have a different type, whereas mutability does not.

## 6. Summary

In this lesson, we covered the concept of shadowing in Rust. Key takeaways include:
- **Shadowing** allows you to declare a new variable with the same name as an existing one, effectively replacing the old variable in that scope.
- **Difference from Mutability**: Unlike mutability, which modifies an existing variable, shadowing creates a new variable that can even have a different type.
- **Scope and Flexibility**: Shadowing can be used in different scopes and allows for flexible data transformation by reusing variable names.

Shadowing is a powerful feature in Rust that, when used appropriately, can make your code more concise and clear. In the next lesson, we will explore more advanced topics that build on the concepts of variables, mutability, and shadowing.
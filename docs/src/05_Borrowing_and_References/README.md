# Borrowing & References

## Overview

In this lesson, we will explore borrowing and references in Rust, key concepts that allow for efficient memory management while ensuring safety. Building on the previous lesson about ownership, we'll see how Rust enforces rules that prevent common programming errors such as null pointer dereferencing and data races, which are prevalent in languages like C and C++. Understanding borrowing and references is essential to writing safe and concurrent Rust code.

## 1. Understanding Borrowing and References

### 1.1 What is Borrowing?

Borrowing in Rust allows you to access a value without taking ownership of it. Instead of transferring ownership, you can "borrow" the value temporarily, which enables you to use it in another part of your program without violating Rust's ownership rules.

### 1.2 Why is Safety Important?

Safety in Rust refers to preventing common programming errors like:
- **Null pointer dereferencing**: Accessing memory that hasn’t been properly initialized.
- **Dangling pointers**: References to memory that has already been freed.
- **Buffer overflows**: Writing data beyond the bounds of allocated memory.
- **Data races**: Concurrent access to memory that leads to unpredictable behavior.

Rust's borrowing system, combined with its strict ownership model, ensures that these issues are avoided at compile time, making Rust programs both safe and efficient.

## 2. Creating References in Rust

### 2.1 Immutable References

An immutable reference allows you to read data without modifying it. When you create an immutable reference, you borrow the value without taking ownership, ensuring that the original data remains unchanged.

#### Example: Immutable Reference

```rust
fn main() {
    let x = 5; // `x` owns the value 5
    let r = &x; // Immutable reference to `x`
    
    println!("Value of x is: {}", x);
    println!("Value of r is: {}", r);
}
```

#### Explanation:
- `x` owns the value `5`.
- `r` is an immutable reference to `x`, meaning it borrows `x` but does not own it. Both `x` and `r` can be used to access the value `5`, but neither can modify it.

### 2.2 Mutable References

A mutable reference allows you to both read and modify the data. However, Rust enforces that you can only have one mutable reference to a value at a time, preventing data races and ensuring memory safety.

#### Example: Mutable Reference

```rust
fn main() {
    let mut x = 5; // `x` is mutable
    let r = &mut x; // Mutable reference to `x`
    
    *r += 1; // Modify the value via the reference
    println!("Value of x is: {}", x);
}
```

#### Explanation:
- `x` is declared mutable with `mut`.
- `r` is a mutable reference to `x`, allowing modification of `x` through `r`.
- The value of `x` is incremented via the mutable reference `r`.

## 3. Rules for Borrowing

Rust enforces strict rules around borrowing to ensure safety and prevent undefined behavior.

### 3.1 Rule 1: Only One Mutable Reference or Many Immutable References

You can have multiple immutable references to a value, but you cannot have a mutable reference if there are any immutable references. This rule prevents data races by ensuring that data cannot be read and modified simultaneously.

#### Example: Conflict Between Immutable and Mutable References

```rust
fn main() {
    let mut x = 5;
    let r1 = &x; // Immutable reference
    let r2 = &x; // Another immutable reference
    
    // let r3 = &mut x; // ERROR: Cannot borrow `x` as mutable because it is already borrowed as immutable
    
    println!("r1: {}, r2: {}", r1, r2);
}
```

#### Explanation:
- `r1` and `r2` are immutable references to `x`. Attempting to create `r3`, a mutable reference, would result in a compilation error because Rust does not allow both mutable and immutable references to coexist.

### 3.2 Rule 2: References Must Always Be Valid

Rust guarantees that references are always valid by preventing them from outliving the data they reference. When a value goes out of scope, any references to it are invalidated, preventing dangling references.

#### Example: Scope and References

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    } // `x` goes out of scope here, `r` is now invalid
    
    // println!("r: {}", r); // ERROR: `x` does not live long enough
}
```

#### Explanation:
- In this example, `x` goes out of scope at the end of the inner block. Since `r` is a reference to `x`, it becomes invalid when `x` is dropped, and Rust prevents you from using `r` outside of `x`'s scope.

## 4. Practical Application: Bank Account Example

To demonstrate borrowing and references in a practical scenario, let’s consider a struct representing a bank account. We will implement methods to manage the account balance while ensuring safe access through borrowing rules.

### 4.1 Struct Definition

```rust
struct BankAccount {
    owner: String,
    balance: f64,
}
```

### 4.2 Implementing Methods with Borrowing

We will create two methods: one for withdrawing money, which requires mutable access, and one for checking the balance, which requires immutable access.

#### Example: Borrowing in Methods

```rust
impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from {}'s account", amount, self.owner);
        self.balance -= amount;
    }
    
    fn check_balance(&self) {
        println!("Account owned by {} has a balance of {}.", self.owner, self.balance);
    }
}
```

#### Explanation:
- `withdraw` takes a mutable reference to `self`, allowing it to modify the account balance.
- `check_balance` takes an immutable reference to `self`, ensuring that the balance is only read, not modified.

### 4.3 Usage in `main`

```rust
fn main() {
    let mut account = BankAccount {
        owner: String::from("Alice"),
        balance: 1050.55,
    };
    
    account.check_balance(); // Immutable borrow
    account.withdraw(50.0);  // Mutable borrow
    account.check_balance(); // Immutable borrow again
}
```

#### Explanation:
- The `main` function first creates an account and then checks the balance using an immutable borrow.
- The `withdraw` method is called with a mutable borrow to update the balance.
- Finally, the balance is checked again using another immutable borrow.
- These operations are safe and compile without errors because the borrowing rules are followed correctly.

## 5. Summary

In this lesson, we covered the fundamental concepts of borrowing and references in Rust. Key points include:
- **Immutable References**: Allow you to read data without modifying it.
- **Mutable References**: Allow you to modify data, but only one mutable reference can exist at a time.
- **Borrowing Rules**: Ensure safety by preventing simultaneous mutable and immutable references, and by ensuring that references are always valid.

Understanding and applying these concepts is crucial for writing safe and efficient Rust code. Borrowing and references are integral to Rust's ability to provide memory safety guarantees without sacrificing performance. The next lesson will explore more advanced topics related to lifetimes and how Rust ensures that references always remain valid within the appropriate scope.
# Borrowing & References in Rust

## Overview

Building upon the foundational concept of ownership, borrowing and references are pivotal in Rust's memory management system. They enable you to access data without taking ownership, facilitating efficient and safe manipulation of data. Understanding these concepts is essential for writing Rust programs that are both performant and free from common memory-related bugs such as null pointer dereferencing, dangling pointers, and data races. This lesson will delve into the principles of borrowing, the distinction between immutable and mutable references, the rules that govern them, and practical applications to solidify your understanding.

---

## 1. Understanding Borrowing and References

Borrowing and references in Rust allow you to access and manipulate data without taking ownership of it. This ensures memory safety by enforcing strict rules that prevent common programming errors.

### 1.1 What is Borrowing?

**Borrowing** is Rust's mechanism for accessing a value without taking ownership. Instead of transferring ownership, you can "borrow" the value temporarily. This allows multiple parts of your program to use the same data without violating Rust's ownership rules.

#### Example: Basic Borrowing

```rust
fn main() {
    let s1 = String::from("Rust");
    let len = calculate_length(&s1); // Borrowing s1
    
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

**Output:**
```
The length of 'Rust' is 4.
```

#### Explanation

- `s1` owns the `String` value `"Rust"`.
- `&s1` creates an immutable reference to `s1`, allowing `calculate_length` to read the value without taking ownership.
- After borrowing, `s1` remains valid in `main`, and its ownership is not transferred.
- This prevents unnecessary copying and ensures efficient memory usage.

### 1.2 Why is Safety Important?

Safety in Rust ensures that your programs are free from common memory-related errors, which are prevalent in languages like C and C++. Rust's borrowing system, combined with its ownership model, enforces the following safety guarantees at compile time:

- **Null Pointer Dereferencing**: Prevents accessing memory that hasn’t been properly initialized.
- **Dangling Pointers**: Eliminates references to memory that has already been freed.
- **Buffer Overflows**: Ensures that data is not written beyond the allocated memory bounds.
- **Data Races**: Prevents concurrent access to mutable data, avoiding unpredictable behavior.

By enforcing these rules, Rust ensures that your programs are both safe and efficient, eliminating entire classes of bugs before your code even runs.

---

## 2. Creating References in Rust

References are pointers that allow you to access data without taking ownership. Rust provides both immutable and mutable references, each serving different purposes and governed by specific rules to ensure safety.

### 2.1 Immutable References

An **immutable reference** allows you to read data without modifying it. Multiple immutable references to the same data are allowed simultaneously, promoting safe concurrent reads.

#### Example: Immutable Reference

```rust
fn main() {
    let x = 5; // `x` owns the value 5
    let r = &x; // Immutable reference to `x`
    
    println!("Value of x is: {}", x);
    println!("Value of r is: {}", r);
}
```

**Output:**
```
Value of x is: 5
Value of r is: 5
```

#### Explanation

- `x` owns the integer value `5`.
- `r` is an immutable reference to `x`, created using `&x`.
- Both `x` and `r` can be used to access the value `5`, but neither can modify it.
- Multiple immutable references can coexist without any issues, as they do not alter the data.

### 2.2 Mutable References

A **mutable reference** allows you to both read and modify the borrowed data. However, Rust enforces that only one mutable reference to a particular piece of data can exist at a time. This rule prevents data races and ensures that data is not inadvertently modified from multiple places simultaneously.

#### Example: Mutable Reference

```rust
fn main() {
    let mut x = 5; // `x` is mutable
    let r = &mut x; // Mutable reference to `x`
    
    *r += 1; // Modify the value via the reference
    println!("Value of x is: {}", x);
}
```

**Output:**
```
Value of x is: 6
```

#### Explanation

- `x` is declared as mutable using `mut`, allowing its value to be changed.
- `r` is a mutable reference to `x`, created using `&mut x`.
- The `*r += 1;` syntax dereferences `r` to modify the value of `x`.
- After modification, printing `x` reflects the updated value.
- Only one mutable reference (`r`) exists at a time, ensuring safe modification.

---

## 3. Rules for Borrowing

Rust enforces strict rules around borrowing to maintain memory safety and prevent undefined behavior. Understanding these rules is crucial for effectively managing references in your programs.

### 3.1 Rule 1: Only One Mutable Reference or Many Immutable References

You can have either:

- **Many immutable references** to a value, allowing multiple parts of your code to read the data simultaneously.
- **One mutable reference** to a value, ensuring exclusive access for modifications.

**But you cannot have both mutable and immutable references to the same value at the same time.**

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

**Compiler Error:**
```
error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:19
  |
4 |     let r1 = &x; // Immutable reference
  |              -- immutable borrow occurs here
5 |     let r2 = &x; // Another immutable reference
6 |     let r3 = &mut x; // ERROR: Cannot borrow `x` as mutable because it is already borrowed as immutable
  |                   ^^^^^^ mutable borrow occurs here
7 | 
8 |     println!("r1: {}, r2: {}", r1, r2);
  |                             -- immutable borrow later used here
```

#### Explanation

- `r1` and `r2` are immutable references to `x`.
- Attempting to create `r3`, a mutable reference, while `r1` and `r2` are still in scope, violates Rust's borrowing rules.
- Rust prevents this to ensure that data is not simultaneously read and modified, avoiding potential data races.

### 3.2 Rule 2: References Must Always Be Valid

Rust ensures that references are always valid by preventing them from outliving the data they point to. When a value goes out of scope, any references to it are invalidated, eliminating the risk of dangling references.

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

**Compiler Error:**
```
error[E0597]: `x` does not live long enough
 --> src/main.rs:4:13
  |
4 |         r = &x;
  |             ^^ borrowed value does not live long enough
5 |     }
  |     - `x` dropped here while still borrowed
6 | 
7 |     println!("r: {}", r); // ERROR: `x` does not live long enough
  |                             ^ borrowed value does not live long enough
```

#### Explanation

- `x` is declared within an inner block and owns the value `5`.
- `r` attempts to borrow `x` outside of its scope.
- Since `x` is dropped at the end of the inner block, `r` would become a dangling reference.
- Rust's compiler detects this and prevents the code from compiling, ensuring that references do not outlive the data they point to.

---

## 4. Practical Application: Bank Account Example

To illustrate borrowing and references in a practical scenario, let's implement a simple bank account system. This example demonstrates how to manage data safely and efficiently using Rust's borrowing rules.

### 4.1 Struct Definition

First, we define a `BankAccount` struct to represent a bank account with an owner and a balance.

```rust
struct BankAccount {
    owner: String,
    balance: f64,
}
```

#### Explanation

- The `BankAccount` struct has two fields:
  - `owner`: A `String` representing the account owner's name.
  - `balance`: A `f64` representing the account balance.

### 4.2 Implementing Methods with Borrowing

Next, we'll implement methods for the `BankAccount` struct that utilize borrowing and references to manage the account balance.

```rust
impl BankAccount {
    // Method to withdraw money; requires a mutable reference to self
    fn withdraw(&mut self, amount: f64) {
        if amount > self.balance {
            println!("Insufficient funds for withdrawal.");
        } else {
            println!("Withdrawing ${} from {}'s account.", amount, self.owner);
            self.balance -= amount;
        }
    }
    
    // Method to check the balance; uses an immutable reference to self
    fn check_balance(&self) {
        println!("Account owned by {} has a balance of ${}.", self.owner, self.balance);
    }
}
```

#### Explanation

- **`withdraw` Method**:
  - Takes a mutable reference to `self` (`&mut self`), allowing it to modify the `balance`.
  - Checks if the withdrawal amount is greater than the current balance.
  - If sufficient funds are available, it deducts the amount from `balance`.
  
- **`check_balance` Method**:
  - Takes an immutable reference to `self` (`&self`), allowing it to read the `balance` without modifying it.
  - Prints the account owner's name and current balance.

### 4.3 Usage in `main`

Finally, we'll use the `BankAccount` struct and its methods in the `main` function to demonstrate borrowing and references in action.

```rust
fn main() {
    let mut account = BankAccount {
        owner: String::from("Alice"),
        balance: 1050.55,
    };
    
    account.check_balance(); // Immutable borrow
    account.withdraw(50.0);  // Mutable borrow
    account.check_balance(); // Immutable borrow again
    
    // Attempting to create a mutable reference while immutable references exist
    // let r1 = &account;
    // let r2 = &mut account; // ERROR: cannot borrow `account` as mutable because it is also borrowed as immutable
}
```

**Output:**
```
Account owned by Alice has a balance of $1050.55.
Withdrawing $50 from Alice's account.
Account owned by Alice has a balance of $1000.55.
```

#### Explanation

- **Creating the Account**:
  - `account` is a mutable instance of `BankAccount`, initialized with owner "Alice" and a balance of `$1050.55`.
  
- **Checking Balance**:
  - `account.check_balance();` borrows `account` immutably to print the current balance.
  
- **Withdrawing Money**:
  - `account.withdraw(50.0);` borrows `account` mutably to deduct `$50` from the balance.
  
- **Rechecking Balance**:
  - `account.check_balance();` borrows `account` immutably again to display the updated balance.
  
- **Commented Code (Optional Error Demonstration)**:
  - The commented-out lines demonstrate an attempt to create both immutable and mutable references simultaneously, which Rust disallows to prevent data races.

---

## 5. Common Errors and Considerations

Understanding Rust's borrowing and reference rules is essential to avoid common programming errors. Below are typical issues developers might encounter when working with borrowing and references, along with explanations and solutions.

### 5.1 Use After Move

When ownership of a value is transferred (moved) to another variable, the original owner can no longer access the value. Attempting to do so results in a compile-time error.

#### Example: Use After Move

```rust
fn main() {
    let s1 = String::from("Rust");
    let s2 = s1; // Ownership moved to s2

    println!("{}", s1); // ERROR: s1 no longer owns the value
}
```

**Compiler Error:**
```
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:20
  |
3 |     let s2 = s1; // Ownership moved to s2
  |              -- value moved here
4 | 
5 |     println!("{}", s1); // ERROR: s1 no longer owns the value
  |                    ^ value borrowed here after move
```

#### Explanation

- `s1` owns the `String` value `"Rust"`.
- `s2 = s1;` transfers ownership to `s2`, rendering `s1` invalid.
- Attempting to use `s1` after the move causes a compile-time error.

#### Solution

If you need to use the value in multiple places, consider **borrowing** or **cloning** the data.

**Using Borrowing:**

```rust
fn main() {
    let s1 = String::from("Rust");
    let s2 = &s1; // Borrowing s1

    println!("s1: {}, s2: {}", s1, s2); // Both are valid
}
```

**Using Cloning:**

```rust
fn main() {
    let s1 = String::from("Rust");
    let s2 = s1.clone(); // Cloning s1

    println!("s1: {}, s2: {}", s1, s2); // Both are valid
}
```

**Note:** Cloning creates a deep copy, which can be expensive for large data structures. Use borrowing when possible to avoid unnecessary overhead.

### 5.2 Dangling References

Rust prevents the creation of **dangling references**—references to memory that has been freed. Attempting to create such references results in compile-time errors.

#### Example: Dangling References

```rust
fn dangle() -> &String {
    let s = String::from("Rust");
    &s
} // `s` goes out of scope here, and `&s` becomes invalid
```

**Compiler Error:**
```
error[E0597]: `s` does not live long enough
 --> src/main.rs:3:20
  |
3 |     &s
  |     ^^ borrowed value does not live long enough
4 | }
  | - `s` dropped here while still borrowed
```

#### Explanation

- The function `dangle` attempts to return a reference to `s`.
- Since `s` is dropped at the end of the function, the reference would point to invalid memory.
- Rust's compiler detects this and prevents the code from compiling.

#### Solution

Return the owned value instead of a reference to ensure that the data remains valid.

```rust
fn no_dangle() -> String {
    let s = String::from("Rust");
    s // Ownership is moved to the caller
}

fn main() {
    let s = no_dangle();
    println!("{}", s);
}
```

**Output:**
```
Rust
```

### 5.3 Immutable and Mutable References

Rust enforces rules to prevent conflicts between immutable and mutable references. Violating these rules leads to compilation errors.

#### Example: Mixing Immutable and Mutable References

```rust
fn main() {
    let mut s = String::from("Rust");

    let r1 = &s; // Immutable reference
    let r2 = &s; // Another immutable reference
    let r3 = &mut s; // Mutable reference

    println!("{}, {}, {}", r1, r2, r3);
}
```

**Compiler Error:**
```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:19
  |
4 |     let r1 = &s; // Immutable reference
  |              -- immutable borrow occurs here
5 |     let r2 = &s; // Another immutable reference
6 |     let r3 = &mut s; // Mutable reference
  |                   ^^^^^^ mutable borrow occurs here
7 | 
8 |     println!("{}, {}, {}", r1, r2, r3);
  |                             -- immutable borrow later used here
```

#### Explanation

- `r1` and `r2` are immutable references to `s`.
- Attempting to create `r3`, a mutable reference, while `r1` and `r2` are still in scope, violates Rust's borrowing rules.
- Rust prevents this to ensure data is not simultaneously read and modified, avoiding potential data races.

#### Solution

Ensure that no immutable references are active when creating a mutable reference. This can be achieved by limiting the scope of immutable references.

```rust
fn main() {
    let mut s = String::from("Rust");

    {
        let r1 = &s; // Immutable reference
        let r2 = &s; // Another immutable reference
        println!("r1: {}, r2: {}", r1, r2);
    } // r1 and r2 go out of scope here

    let r3 = &mut s; // Mutable reference
    r3.push_str(" is awesome!");
    println!("{}", r3);
}
```

**Output:**
```
r1: Rust, r2: Rust
Rust is awesome!
```

**Explanation**

- The immutable references `r1` and `r2` are confined within an inner block.
- Once the inner block ends, `r1` and `r2` go out of scope, freeing up `s` for a mutable reference.
- `r3` is then created as a mutable reference, allowing modification of `s`.

### 5.4 Multiple Mutable References in Different Scopes

Rust allows multiple mutable references as long as they are in different scopes, ensuring that they do not coexist and violate the borrowing rules.

#### Example: Multiple Mutable References in Different Scopes

```rust
fn main() {
    let mut s = String::from("Rust");

    {
        let r1 = &mut s; // First mutable reference
        r1.push_str(" is powerful!");
        println!("{}", r1);
    } // r1 goes out of scope here

    let r2 = &mut s; // Second mutable reference
    r2.push_str(" and safe.");
    println!("{}", r2);
}
```

**Output:**
```
Rust is powerful!
Rust is powerful! and safe.
```

#### Explanation

- `r1` is a mutable reference within the first inner block. It modifies `s` and goes out of scope after the block.
- After `r1` goes out of scope, `r2` is created as another mutable reference, which further modifies `s`.
- Since `r1` and `r2` do not coexist, Rust allows this pattern without errors.

---

## 6. Summary

Borrowing and references are integral to Rust's ownership model, enabling efficient and safe memory management without the need for a garbage collector. This tutorial covered:

- **Immutable References**: Allowing multiple parts of your code to read data without modifying it.
- **Mutable References**: Allowing a single part of your code to modify data, ensuring exclusive access.
- **Borrowing Rules**: Enforcing that you cannot have both mutable and immutable references simultaneously and that references must always be valid.
- **Practical Applications**: Demonstrating how to implement borrowing and references through a `BankAccount` example.
- **Common Errors and Solutions**: Identifying typical mistakes and providing strategies to resolve them.

### Key Takeaways

- **Memory Safety**: Rust's borrowing and reference rules prevent common memory-related bugs, ensuring that your programs are safe and reliable.
- **Efficient Data Access**: Borrowing allows you to access data without unnecessary copying, optimizing performance.
- **Concurrency Safety**: By enforcing exclusive mutable references and allowing multiple immutable references, Rust ensures safe concurrent access to data, eliminating data races.
- **Compiler Enforcement**: Rust's compiler rigorously checks borrowing rules at compile time, catching potential errors early in the development process.

### Next Steps

Building upon your understanding of borrowing and references, future lessons will explore more advanced Rust concepts, including:

- **Lifetimes**: Managing the scope and validity of references to ensure that they do not outlive the data they point to.
- **Advanced Borrowing**: Handling complex borrowing scenarios, such as nested references and borrowing in structs.
- **Smart Pointers**: Utilizing Rust's smart pointer types like `Box`, `Rc`, and `RefCell` for advanced memory management.
- **Concurrency**: Leveraging Rust's ownership and type system to write safe and efficient multi-threaded programs.
- **Error Handling**: Implementing robust error handling strategies using `Result` and `Option` types.

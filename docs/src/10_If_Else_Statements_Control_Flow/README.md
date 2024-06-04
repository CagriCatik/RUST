
# Loops / Control Flow

In this tutorial, we will delve into the essential concept of loops in Rust programming language. Loops are crucial for repeating a block of code multiple times, and Rust offers three types of loops: `loop`, `while`, and `for` loops. We will explore each of these loops, along with practical examples and code snippets.

**1. The `loop` Keyword:**
The `loop` keyword in Rust instructs the program to execute a block of code repeatedly until explicitly told to stop. It's an unconditional loop, meaning it will continue running indefinitely until instructed otherwise.

```rust
fn main() {
    loop {
        println!("Hello, world!");
        // To stop the loop manually, use Ctrl+C
    }
}
```

**2. Returning Values from a Loop:**
One common use case of loops is to retry an operation that might fail, such as checking whether a thread has completed its job. We can also pass the result of the operation out of the loop. The `break` expression is used to halt the loop execution and return a value.

```rust
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result: {}", result); // Output: 20
}
```

**3. Loop Labels:**
When dealing with nested loops, where one loop is inside another, loop labels become useful. By default, `break` and `continue` statements apply to the innermost loop. We can use loop labels to specify which loop to break or continue.

```rust
fn main() {
    'counting_up: loop {
        let mut count = 0;
        println!("Count: {}", count);
        let mut remaining = 10;
        'inner: loop {
            println!("Remaining: {}", remaining);
            if remaining == 9 {
                break 'counting_up;
            }
            count += 1;
            remaining -= 1;
        }
    }
}
```

**4. Conditional Loops with `while`:**
The `while` loop runs the loop as long as a condition holds true. It evaluates the condition in each iteration, and when the condition becomes false, the loop stops. We can implement complex behaviors using a combination of `loop`, `if else`, and `break`.

```rust
fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("Hey!");
}
```

**5. Looping Through Collections with `for` Loop:**
The `for` loop in Rust iterates over each item in a collection. It is commonly used to loop through arrays, vectors, iterators, etc.

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5, 6];
    for element in arr.iter() {
        println!("{}", element);
    }

    let strings = ["a", "b", "c", "d", "e", "f"];
    for letter in strings.iter() {
        println!("{}", letter);
    }
}
```

**Conclusion:**
In this lesson, we covered the fundamentals of loops in Rust, including the `loop`, `while`, and `for` loops. Loops are powerful constructs for executing code repeatedly, and mastering them is essential for any Rust programmer. Experiment with the provided examples to solidify your understanding, and stay tuned for more lessons. Thank you for learning with us!

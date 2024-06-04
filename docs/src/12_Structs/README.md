# Understanding and Implementing Structs in Rust

In this tutorial, we'll delve into the concept of structs in Rust programming language. Structs are fundamental data structures used to encapsulate related data fields. They resemble tuples but with named fields, providing clarity and expressiveness to your code. Let's explore how to define, instantiate, and manipulate structs in Rust.

## 1. Introduction to Structs

Structs in Rust are analogous to tuples but offer named fields for better readability and organization. Each field can have a different data type, making structs versatile and powerful.

## Syntax:

```rust
struct StructName {
    field1: Type1,
    field2: Type2,
    // Add more fields as needed
}
```

## 2. Creating Structs

Let's create two structs: `Book` and `User`, to demonstrate the concept.

```rust
// Define Book struct
struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}

// Define User struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

## 3. Instantiating Structs

You can create instances of structs by providing values for each field.

```rust
// Instantiate a Book struct
let my_book = Book {
    title: String::from("Rust Programming"),
    author: String::from("John Doe"),
    pages: 200,
    available: true,
};

// Instantiate a User struct
let user1 = User {
    active: true,
    username: String::from("example_user"),
    email: String::from("user@example.com"),
    sign_in_count: 1,
};
```

## 4. Accessing and Modifying Struct Fields

You can access and modify struct fields using dot notation. Remember, the entire struct instance must be mutable to modify any of its fields.

```rust
// Accessing and modifying struct fields
let mut user2 = User {
    active: false,
    username: String::from("another_user"),
    email: String::from("another@example.com"),
    sign_in_count: 0,
};

// Modify email field
user2.email = String::from("new_email@example.com");
```

## 5. Returning Structs from Functions

Functions can return structs, providing a convenient way to create and initialize structs.

```rust
// Define a function to build a User struct
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email, // field init shorthand
        username,
        sign_in_count: 1,
    }
}

// Call the function to create a User struct
let user3 = build_user(String::from("user3@example.com"), String::from("user3"));
```

## 6. Creating Instances from Other Instances

You can create new instances of a struct by using values from existing instances.

```rust
// Create a new User instance based on user1 but with a different email
let user4 = User {
    email: user1.email, // Reuse email from user1
    ..user1 // Reuse other fields from user1
};
```

## 7. Tuple Structs and Unit-Like Structs

Tuple structs are similar to normal structs but without named fields. Unit-like structs have no fields and are used for implementing traits without storing data.

## Tuple Struct:

```rust
// Define a tuple struct
struct Color(i32, i32, i32);

// Instantiate tuple structs
let black = Color(0, 0, 0);
let white = Color(255, 255, 255);
```

## Unit-Like Struct:

```rust
// Define a unit-like struct
struct AlwaysEqual;

// Instantiate unit-like struct
let always = AlwaysEqual;
```
 

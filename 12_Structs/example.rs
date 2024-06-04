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

// Function to build a User struct
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email, // field init shorthand
        username,
        sign_in_count: 1,
    }
}

fn main() {
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

    // Accessing and modifying struct fields
    let mut user2 = User {
        active: false,
        username: String::from("another_user"),
        email: String::from("another@example.com"),
        sign_in_count: 0,
    };

    // Modify email field
    user2.email = String::from("new_email@example.com");

    // Create a new User instance based on user1 but with a different email
    let user3 = User {
        email: user1.email, // Reuse email from user1
        ..user1 // Reuse other fields from user1
    };

    // Call the function to create a User struct
    let user4 = build_user(String::from("user4@example.com"), String::from("user4"));

    // Print out the details of the users
    println!("Book Details:");
    println!("Title: {}", my_book.title);
    println!("Author: {}", my_book.author);
    println!("Pages: {}", my_book.pages);
    println!("Available: {}", my_book.available);
    println!();

    println!("User 1 Details:");
    println!("Active: {}", user1.active);
    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Sign-in Count: {}", user1.sign_in_count);
    

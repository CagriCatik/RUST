// Rust code for Compound Data Types

// Define a struct to represent a book
struct Book {
    title: String,
    author: String,
    genre: String,
    available: bool,
}

impl Book {
    // Constructor method to create a new book instance
    fn new(title: &str, author: &str, genre: &str, available: bool) -> Self {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            genre: genre.to_string(),
            available,
        }
    }

    // Method to borrow a book
    fn borrow(&mut self) -> () {
        if self.available {
            println!("You have borrowed '{}' by {}", self.title, self.author);
            self.available = false;
        } else {
            println!("Sorry, '{}' is currently unavailable.", self.title);
        }
    }

    // Method to return a book
    fn return_book(&mut self) {
        println!("You have returned '{}'. Thank you!", self.title);
        self.available = true;
    }
}

fn main() {
    // Create an array of books
    let mut library: Vec<Book> = vec![
        Book::new("1984", "George Orwell", "Dystopian", true),
        Book::new("To Kill a Mockingbird", "Harper Lee", "Fiction", true),
        Book::new("The Great Gatsby", "F. Scott Fitzgerald", "Classic", false),
    ];

    // Display available books in the library
    println!("Available Books:");
    for book in &library {
        if book.available {
            println!("Title: {}, Author: {}, Genre: {}", book.title, book.author, book.genre);
        }
    }

    // Borrow a book
    library[0].borrow();

    // Display available books after borrowing
    println!("\nAvailable Books after borrowing:");
    for book in &library {
        if book.available {
            println!("Title: {}, Author: {}, Genre: {}", book.title, book.author, book.genre);
        }
    }

    // Return the borrowed book
    library[0].return_book();

    // Display available books after returning
    println!("\nAvailable Books after returning:");
    for book in &library {
        if book.available {
            println!("Title: {}, Author: {}, Genre: {}", book.title, book.author, book.genre);
        }
    }
}

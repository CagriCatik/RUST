# Rust Fundamentals

![Rust Logo](https://www.rust-lang.org/logos/rust-logo-blk.svg)

Rust is a modern, statically-typed systems programming language that emphasizes safety, concurrency, and performance. This repository serves as a comprehensive guide to the fundamentals of Rust, covering everything from primitive data types to advanced concepts like ownership, borrowing, and control flow.

## Table of Contents

1. [ ] [Primitive Data Types](./01_Primitive_Data_Types/README.md)
2. [ ] [Compound Data Types](./02_Compound_Data_Types/README.md)
3. [ ] [Functions](./03_Functions/README.md)
4. [ ] [Ownership](./04_Ownership/README.md)
5. [ ] [Borrowing &amp; References](./05_Borrowing_and_References/README.md)
6. [ ] [Variables &amp; Mutability](./06_Variables_&_Mutability/README.md)
7. [ ] [Constants](./07_Constants/README.md)
8. [ ] [Shadowing](./08_Shadowing/README.md)
9. [ ] [Comments](./09_Comments/README.md)
10. [ ] [If-Else Statements - Control Flow](./10_If_Else_Statements_Control_Flow/README.md)
11. [ ] [Loops Control - Flow](./11_Loops-Control-Flow/README.md)
12. [ ] [Structs](./12_Structs/README.md)
13. [ ] [Enums](./13_Enums/README.md)
14. [ ] [Error Handling Techniques](./14_Error-Handling/README.md)
15. [ ] [Collection Types](./15_Collection-Types/README.md)
16. [ ] [Projects](./16_Projects/README.md)


## Getting Started

To get started with Rust, you'll need to [install the Rust toolchain](https://www.rust-lang.org/tools/install) on your system. Once installed, you can compile and run Rust programs using the `rustc` compiler and the `cargo` build tool.

## Run the Rust Code


1. **Create a New Rust Project:**
   - Navigate to the directory where you want to create your project.
   - Use the `cargo` command to create a new project. For example, to create a project named `hello_world`, run:
     ```sh
     cargo new <project_name>
     ```
   - This command creates a new directory named `<project_name>` with a basic project structure.


2. **Build the Project:**
   - In the project directory, run the following command to build your project:
     ```sh
     cargo build
     ```
   - This compiles your project and generates an executable in the `target/debug` directory.

3. **Run the Project:**
   - To run the project, use the following command:
     ```sh
     cargo run
     ```
   - This will compile and run your project in one step, displaying the output.


## Learning Resources

This guide is based on the excellent [Bek Brace YouTube channel](https://www.youtube.com/playlist?list=PLrOQsSoS-V69UWKxV4FNRJFlHS0DUFQA2), which provides comprehensive video tutorials on Rust fundamentals. Additionally, you can refer to the official [Rust Book](https://doc.rust-lang.org/book/) and the [Rust by Example](https://doc.rust-lang.org/rust-by-example/) for in-depth documentation and examples.


### Conclusion

By following these steps, you should be able to set up, write, build, and run Rust code on your local machine. Rust's tooling, especially with `cargo`, makes managing projects and dependencies straightforward and efficient.
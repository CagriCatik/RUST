fn main() {
    // Arrays
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number array: {:?}", numbers);

    // Accessing array elements
    println!("First number: {}", numbers[0]);
    println!("Second number: {}", numbers[1]);
    println!("Third number: {}", numbers[2]);

    // Attempt to mix data types in an array (This will cause a compilation error if uncommented)
    // let mix = [1, "Apple", true];
    // println!("Mix array: {:?}", mix);

    // Tuples
    let person: (&str, i32, bool) = ("Alice", 30, false);
    println!("Person tuple: {:?}", person);

    // Accessing tuple elements using pattern matching
    let (name, age, is_student) = person;
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Is student: {}", is_student);

    // Accessing tuple elements using indexing
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is student: {}", person.2);

    // Slices
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let number_slice: &[i32] = &numbers[1..4];
    println!("Number slice: {:?}", number_slice);

    // String and String Slices
    let mut greeting = String::from("Hello");
    greeting.push_str(", world!");
    println!("{}", greeting);

    let greeting = String::from("Hello, world!");
    let hello = &greeting[0..5];
    println!("Slice: {}", hello);

    // Demonstrating difference between String and &str
    let string = String::from("Hello, world!");
    let string_slice: &str = &string;
    println!("String: {}", string);
    println!("String slice: {}", string_slice);
}

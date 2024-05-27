fn main() {
    // Integer examples
    let signed_int: i32 = -42;
    let unsigned_int: u64 = 100;

    println!("Signed integer: {}", signed_int);
    println!("Unsigned integer: {}", unsigned_int);

    // Demonstrating the range of i32 and i64
    let max_i32: i32 = i32::MAX;
    let max_i64: i64 = i64::MAX;

    println!("Maximum value of i32: {}", max_i32);
    println!("Maximum value of i64: {}", max_i64);

    // Uncommenting the following line will cause a compile-time error due to overflow
    // let overflow_i32: i32 = 2_147_483_648;

    // Floating-point number examples
    let pi: f64 = 3.141592653589793;
    let euler: f32 = 2.71828;

    println!("Value of Pi (f64): {}", pi);
    println!("Value of Euler's number (f32): {}", euler);

    // Boolean examples
    let is_raining: bool = false;
    let is_sunny: bool = true;

    println!("Is it raining? {}", is_raining);
    println!("Is it sunny? {}", is_sunny);

    // Character examples
    let letter_a: char = 'A';
    let heart_emoji: char = '❤';

    println!("First letter of the alphabet: {}", letter_a);
    println!("Heart emoji: {}", heart_emoji);

    // Exploring range and overflow behavior
    println!("Attempting to assign out-of-range values:");

    // Uncommenting the following lines will cause compile-time errors
    // let out_of_range_i32: i32 = 2_147_483_648; // i32 max value is 2_147_483_647
    // let out_of_range_u64: u64 = -1; // u64 cannot be negative

    // Correcting the out-of-range example with appropriate types
    let correct_i128: i128 = 2_147_483_648;
    let correct_u64: u64 = 1;

    println!("Corrected value fitting i128: {}", correct_i128);
    println!("Corrected positive u64 value: {}", correct_u64);
}

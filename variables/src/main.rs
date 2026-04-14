use std::io;

// Declared in global scope, so it is a constant and cannot be changed
const SERVER_NAME: &str = "localhost";

fn main() {
    /* Common programming concepts in Rust */
    let mut number = 5;
    println!("The number is: {number}");
    number = 10;
    println!("The number is: {number}");
    println!("The server name is: {SERVER_NAME}");
    shadowing_example();
    data_types_example();
    print_months();
    read_array_index();
}

fn shadowing_example() {
    /* Example of variable shadowing */
    let number = 1;
    println!("The number is {number}");
    let number = number + 1;
    {
        // The value of number is 2 in this scope, but it is not changed in the outer scope
        let number = number * 2;
        println!("The number is {number}");
    }
    println!("The number is {number}");
}

fn data_types_example() {
    /* Print examples of different data types */
    let small_integer: i8 = 127;
    println!("Small integer: {small_integer}");
    let large_integer: i64 = 1234567890123456789;
    println!("Large integer: {large_integer}");
    let floating_point: f64 = 3.14;
    println!("Floating point: {floating_point}");
    let boolean: bool = true;
    println!("Boolean: {boolean}");
    // Compared to strings, char uses single quotes and represents a single Unicode scalar value, which can be a letter, a number, a symbol, or even an emoji
    let character: char = 'A';
    println!("Character: {character}");
    //Include other scalar types
    let decimal = 10_000;
    println!("Decimal: {decimal}");
    let hexadecimal = 0xff;
    println!("Hexadecimal: {hexadecimal}");
    let octal = 0o77;
    println!("Octal: {octal}");
    let binary = 0b10_0101;
    println!("Binary: {binary}");
    let byte = b'B';
    println!("Byte: {byte}");
    //compound types
    let tuple: (i32, f64, char) = (42, 3.14, 'R');
    let (x, y, z) = tuple; // Destructuring the tuple into individual variables
    println!("Tuple values: x = {x}, y = {y}, z = {z}");
    println!("Tuple: {tuple:?}");
    let (x, y, z) = (tuple.0, tuple.1, tuple.2); // Accessing tuple elements directly
    println!("Tuple[0] -> {x}, Tuple[1] -> {y}, Tuple[2] -> {z}");
}

fn print_months() {
    /* Print all months */
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    for month in months {
        println!("{month}");
    }
    let message = "Months of the year";
    let number = 12;
    let number: &str = &number.to_string(); // Convert the number to a string slice
    // Array content must match the same type, so we convert the number to a string slice before creating the array
    let new_array = [message, number];
    println!("New array: {new_array:?}");
}

fn read_array_index() {
    /* Read an array index from user input and print the corresponding element */
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

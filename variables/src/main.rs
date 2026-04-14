// Declared in global scope, so it is a constant and cannot be changed
const SERVER_NAME: &str = "localhost";
fn main() {
    let mut number = 5;
    println!("The number is: {number}");
    number = 10;
    println!("The number is: {number}");
    println!("The server name is: {SERVER_NAME}");
    shadowing_example();
}

fn shadowing_example() {
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

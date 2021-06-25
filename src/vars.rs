// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block scoped language

pub fn run() {
    let name = "Brad";
    let mut age = 37;

    // to remove warning about unused variable
    println!("my name is {} and i am {} years old", name, age);

    age = 38;
    println!("my name is {} and i am {} years old", name, age);

    // define constant
    const ID: i32 = 001;

    println!("ID {}", ID);

    // assign multiple variables
    let (my_name, my_age) = ("Brad", 37);
    println!("my name is {} my age is {}", my_name, my_age);

}
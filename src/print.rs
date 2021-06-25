pub fn run() {
    // print to concole
    println!("hello to the console");
    
    // basic formatting
    println!("{}", 1);
    println!("{} is from {}", "Brad", "Mass");

    // positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    // named arguments
    println!("{name} likes to play {activity}", name = "John", activity = "Baseball");

    // placeholder traits
    println!("Binary: {0:b} Hex: {0:x} Octal: {0:o}", 10);

    // placeholder debug trait
    println!("{:?}", (12, true, "Hello")); // touple

    // basic math
    println!("10 + 10 = {}", 10+10);
}
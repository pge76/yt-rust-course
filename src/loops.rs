// loops used to iterate until condition is met

pub fn run() {

    let mut count = 0;

    
    /*
    loop {
        count += 1;
        println!("num: {}", count);
        if count == 20 { break };
    }
    */

    // while loop FizzBuzz

    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", count)
        }
        count += 1;
    }

    for x in 1 .. 100 {
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", x)
        }
    }

}
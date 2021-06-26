pub fn run() {
    greeting("hello", "jane");
    println!("{}", add(1, 2));

    // bind function value to variable
    let get_sum = add(5,5);
    println!("sum {}", get_sum);

    // closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3; // can use variable from outer block
    println!("csum {}", add_nums(2,3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
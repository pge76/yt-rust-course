// vectors are variable length collections with the same datatype

use std::mem;

pub fn run() {
    let mut numbers: Vec<i64> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);
    println!("single value {}", numbers[0]);

    // length
    println!("{}", numbers.len());

    // vectors are stack allocated
    println!("vector element byte size {}", mem::size_of_val(&numbers[0]));
    println!("vector byte size {}", mem::size_of_val(&numbers));

    // slice
    let slice: &[i64] = &numbers[0..2];
    println!("slice: {:?}", slice);

    // push
    numbers.push(6);
    numbers.push(7);
    println!("{:?}", numbers);

    // loop
    for x in numbers.iter() {
        println!("{}", x);
    }

    // loop and mutate
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("{:?}", numbers);

}
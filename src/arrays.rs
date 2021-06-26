// arrays are fixed length collections with the same datatype

use std::mem;

pub fn run() {
    let numbers: [i64; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);
    println!("single value {}", numbers[0]);

    let mut numbers_mut: [i32; 5] = [1, 2, 3, 4, 5];

    numbers_mut[4] = 20;
    println!("{:?}", numbers_mut);
    println!("single value {}", numbers_mut[4]);

    // length
    println!("{}", numbers.len());

    // arrays are stack allocated
    println!("array element byte size {}", mem::size_of_val(&numbers[0]));
    println!("array byte size {}", mem::size_of_val(&numbers));

    // slice
    let slice: &[i64] = &numbers[0..2];
    println!("slice: {:?}", slice);
}
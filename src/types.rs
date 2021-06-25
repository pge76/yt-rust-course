/* Primitive Types--

Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean: bool
Character: char
Tuples
Arrays
*/

// rust is a statically typed language, which means that it must know the types of all
// variables at compile time, however the compiler can usually infer what type we want to use
// based on the value and how we use it

pub fn run() {
    // default is "i32"
    let _x = 1;

    // default is "f64"
    let _y = 2.5;

    // explicit type
    let _a: i64 = 23232323232323232;

    // find max size
    println!("max i32 {}", std::i32::MAX);
    println!("max u64 {}", std::u64::MAX);

    // boolean
    let is_active = true;
    println!("{:?}", is_active);

    // get boolean from expression
    let is_greater = 10 > 9;
    println!("{:?}", is_greater);

    let c = 'c';
    let face = '\u{1F600}';
    println!("{:?}", (c, face));

}
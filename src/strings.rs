// primitve str = immutable fixed-length string somewhere in memory
// String = growable, heap-allocated data structure - use when you need to modify or own string data

pub fn run() {

    // immutable
    let hello = "Hello, world!";

    let mut hello_mutable = String::from("Hello, world!");
    hello_mutable.push_str(" Bye!");


    // get length
    println!(".len() {:?}", (hello_mutable.len()));

    // capacity in bytes
    println!(".capacity() {:?}", (hello_mutable.capacity()));

    // is Empty?
    println!(".is_empty() {:?}", (hello_mutable.is_empty()));   

    // contains
    println!(".contains(\"World\") {:?}", (hello_mutable.contains("world")));

    // contains
    println!(".replace() {:?}", (hello_mutable.replace("world", "there")));

    for word in hello_mutable.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    assert_eq!(s.len(), 2);
    assert_eq!(s.capacity(), 10);

    println!("{:?}", (hello, hello_mutable, s));

}
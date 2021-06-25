// touples group togher values of different types
// max 12 elements

pub fn run() {
    let person: (&str, &str, i8, String) = ("Brad", "Mass", 57, String::from("Wales"));
    println!("{} is from {} and is {} Land: {}", person.0, person.1, person.2, person.3);
}
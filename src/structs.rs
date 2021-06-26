// structs - used to create custom data types
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// tuple struct
struct ColorT(u8, u8, u8);

// function struct
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // construct a person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(), 
            last_name: last.to_string() 
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}


pub fn run() {

    let mut c = Color { 
        red: 255, 
        green: 0, 
        blue: 255
    };

    let mut d = ColorT(12, 10, 129);

    c.green = 128;
    d.2 = 220;

    println!("Color {} {} {}", c.red, c.green, c.blue);
    println!("ColorT {} {} {}", d.0, d.1, d.2);

    let mut p = Person::new("John", "Doe");
    //p.last_name = "Papa Joe".to_string();
    p.set_last_name("Papa Joe");
    println!("Person {} {}", p.first_name, p.last_name);
    println!("Full Name {}", p.full_name());

    println!("tuple name {:?}", p.to_tuple());
}
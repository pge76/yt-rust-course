pub fn run() {

    let age:u8 = 18;
    let check_id:bool = false;
    let knows_person_of_age = true;

    // if else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: age is ok");
    } else if age < 21 && check_id {
        println!("Bartender: bye bye");
    } else {
        println!("Bartender: give id pls");        
    }

    // short if
    let is_of_age = if age >= 21 { true } else { false };

    println!("is of age {}", is_of_age);

}
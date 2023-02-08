pub fn run() {
    let age: u8 = 18;
    let check_id = false;

    if age >= 21 && check_id {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry you gotta leave.");
    } else {
        println!("Bartender: May I see your ID?");
    }
    //same logic operators as C

    //shorthand
    let is_of_age = if age >= 21 {true} else {false};
    println!("Is of age? {}", is_of_age);
}
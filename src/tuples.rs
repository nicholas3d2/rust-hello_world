pub fn run() {
    //Group values, up to 12 elements
    let person: (&str, &str, i8) = ("Nick", "Ontario", 21);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}


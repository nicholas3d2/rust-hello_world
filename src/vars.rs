//Vars primitive like in C, immutable by default, block-scoped

pub fn run() {
    let name = "Nick";
    let mut age = 20; //need mut for mutable variable!!!

    println!("My name is {} and I'm {}", name, age);
    age = 21;
    println!("I will be {}", age);

    let (my_name, my_age) = ("Nick", 20);
    println!("{} : {}", my_name, my_age);
}
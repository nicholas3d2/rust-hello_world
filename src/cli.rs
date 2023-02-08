use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    //cargo run <string1, string2, etc> we will get a vector of args seperated by space!
    println!("Args: {:?}", args);

    let command = args[1].clone(); //args[0] is going to always be the target program running!
    println!("Command: {}", command);

    //if we get hello as first arg, reply
    let name = "Nick";
    let status = "100%";
    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("Invalid command!");
    }
}
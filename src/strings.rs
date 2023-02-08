pub fn run() {
    //Immutable strings: str
    let _hello = "Hello";
    
    //Growable, heap allocated data struct: String
    let mut hello_string = String::from("Hello ");
    println!("Length: {}", hello_string.len());

    hello_string.push('W'); //push only for a single char
    hello_string.push_str("orld!"); //push a string

    println!("Capacity: {}", hello_string.capacity());
    println!("Is Empty: {}", hello_string.is_empty());
    println!("Contains 'World': {}", hello_string.contains("World"));

    println!("{}", hello_string);

    println!("Replace: {}", hello_string.replace("World", "There"));

    //Loop through words by whitespace
    for word in hello_string.split_whitespace() {
        println!("{}", word);
    }

    let mut s = String::with_capacity(10); //size 10

    s.push('a');
    s.push('b');

    println!("Capacity string: {}", s);

    //Assertion testing!!!
    assert_eq!(2, s.len());
    //assert_eq!(3, s.len());

}
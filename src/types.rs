pub fn run() {
    //default is i32
    let x = 1;

    //default is f64
    let y = 2.1;

    //if we want it explicit
    let z: i64 = 1234617264;

    //find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolean
    let is_active = true;

    //Char
    let a1 = 'a';

    //String
    let str = "abc";

    println!("{:?}", (x, y, z, is_active, a1, str));
}
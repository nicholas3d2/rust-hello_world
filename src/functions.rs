pub fn run() {
    greeting("Hello", "John");

    //bind function values to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    //closure
    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3; //can add n3 into here without needing it passed as an arg!
    println!("C Sum: {}", add_nums(3,3));
}

//void
fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

//return
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
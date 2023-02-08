use std::mem; //import mem library

pub fn run() {
    //vectors are resizable arrays (dynamic struct)
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    println!("{:?}", numbers); //print all numbers
    println!("Index 0 is {}", numbers[0]); //index 0

    numbers[0] = 20;
    numbers.push(5);
    numbers.push(6);
    println!("Index 0 is {}", numbers[0]); //index 0
    println!("Length is {}", numbers.len());
    numbers.pop();
    println!("Length is {}", numbers.len());
    println!("Vector is {} bytes", mem::size_of_val(&numbers)); //stack allocated, so we can check bytes size of vector

    let slice: &[i32] = &numbers[0..2]; //assign by ref, only want first 2 entries (0,1)
    println!("Subset array: {:?}", slice);

    //loop through values
    for x in numbers.iter() {
        println!("{}", x);
    }

    //loop and mutate
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers vec: {:?}", numbers);

}

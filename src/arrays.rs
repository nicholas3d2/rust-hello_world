use std::mem; //import mem library

pub fn run() {
    //arrays are a fixed list, all same data type (vectors are for growable arrays)
    //specify type; length
    //mutatable need to add mut
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    println!("{:?}", numbers); //print all numbers
    println!("Index 0 is {}", numbers[0]); //index 0

    numbers[0] = 20;
    println!("Index 0 is {}", numbers[0]); //index 0
    println!("Length is {}", numbers.len());
    println!("Array is {} bytes", mem::size_of_val(&numbers)); //stack allocated, so we can check bytes size of array

    let slice: &[i32] = &numbers[0..2]; //assign by ref, only want first 2 entries (0,1)
    println!("Subset array: {:?}", slice)
}

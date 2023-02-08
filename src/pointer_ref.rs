pub fn run() {
    //primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;
    
    println!("Values: {:?}", (arr1, arr2));
    //with non primitives, if you assign first variable, it will not hold value, need to use &

    //vectors (not primitive values!)
    let vec1 = vec![1,2,3];
    let vec2 = &vec1; //CREATE REFERENCE

    println!("Vec values: {:?}", (&vec1, vec2)); //also need & for printing contents of vec

}
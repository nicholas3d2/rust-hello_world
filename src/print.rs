pub fn run() {
    println!("Hello from print.rs file!");

    //Basic formatting
    println!("Number: {}", 1);

    println!("{} is {}", "SauaS", "FuF");

    //Positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Nick", "GrurG", "ReeR");

    //Named args
    println!("{name} likes to play {activity}", name = "Matthew", activity = "baseball");

    //Traits
    println!("Bin: {:b} Hex: {:x} Oct: {:o}", 10, 10, 10);

    //Debug trait
    println!("{:?}", (12, true, "hello"));
}
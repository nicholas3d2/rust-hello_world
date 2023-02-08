//traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

//tuple struct
struct ColorTup(u8, u8, u8);

struct Person{
    first_name: String,
    last_name: String
}

impl Person {
    //constructor
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn get_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    println!("Color 1: R:{} G:{} B:{}", c.red, c.green, c.blue);

    c.red = 200;
    c.green = 100;

    println!("Color 2: R:{} G:{} B:{}", c.red, c.green, c.blue);

    let c_tup = ColorTup(0, 0, 255);

    println!("Color 3: R:{} G:{} B:{}", c_tup.0, c_tup.1, c_tup.2);

    let p = Person::new("John", "Doe");
    println!("Person 1 {}", p.get_name());

    let mut p2 = Person::new("Mary", "Williams");
    p2.set_last_name("Doe");
    println!("Person 2 {}", p2.get_name());
    println!("Person 2 Tuple {:?}", p2.to_tuple());
}

struct RustDev {
    awesome: bool
}

struct JavaDev {
    awesome: bool
}

struct CDev {

}

trait Developer {
    fn new(awesome: bool) -> Self;
    fn language(&self) -> &str;
    fn say_hello(&self) { println!("Hello. {}", stringify!(Developer)); }
}

impl Developer for RustDev {
    fn new(awesome: bool) -> Self {
        RustDev { awesome }
    }
    fn language(&self) -> &str {
        "Rust"
    }
    fn say_hello(&self) { println!("Hello. Awesome? {}", self.awesome); }
}

impl Developer for JavaDev {
    fn new(awesome: bool) -> Self {
        JavaDev { awesome }
    }
    fn language(&self) -> &str {
        "Java"
    }
    fn say_hello(&self) { println!("Hello. Awesome? {}", self.awesome); }
}

impl Developer for CDev {
    fn new(_awesome: bool) -> Self {
        CDev { }
    }
    fn language(&self) -> &str {
        "C"
    }
}

// Trait Generics
trait Bark {
    fn bark(&self) -> String;
}

struct Dog {
    species: &'static str
}

struct Cat {
    color: &'static str
}

impl Bark for Dog {
    fn bark(&self) -> String {
        format!("{} says woof!", self.species)
    }
}

impl Cat {
    fn meow(&self)  {
        println!("{} cat says meow!", self.color)
    }
}

fn bark_it<T: Bark>(dev: T) {
    println!("{}", dev.bark());
}


// Returning Traits
// The compiler needs to know the space required for a function return type.
// Workaround: return a "box" with a "dyn" trait
struct Dog2 {}
struct Cat2 {}

trait Animal2 {
    fn speak(&self) -> &'static str;
}

impl Animal2 for Dog2 {
    fn speak(&self) -> &'static str {
        "woof"
    }
}

impl Animal2 for Cat2 {
    fn speak(&self) -> &'static str {
        "meow"
    }
}

fn get_animal(rand_number: f64) -> Box<dyn Animal2> {
    if rand_number < 0.5 {
        Box::new(Dog2 {})
    } else {
        Box::new(Cat2 {})
    }
}

// A trait can be added to a struct we have not created
trait MySummable<T>{
    fn sum(&self) -> T;
}

impl MySummable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut sum = 0;
        for i in self {
            sum += i;
        }
        sum
    }
}

// Operator overloading
// Operators are implemented as traits
use std::ops::Add;
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

fn main() {
    let rust_dev = RustDev::new(true);
    let java_dev = JavaDev::new(false);
    let c_dev = CDev::new(false);

    rust_dev.say_hello();
    java_dev.say_hello();
    c_dev.say_hello();

    println!("I love {}!", rust_dev.language());
    println!("I hate {}!", java_dev.language());
    println!("I hate {}!", c_dev.language());

    let dog = Dog { species: "Iggy" };
    let cat = Cat { color: "Blue" };
    bark_it(dog);
    // bark_it(cat); // NOT IMPLEMENTED!
    cat.meow();

    // Returning Traits example
    let animal = get_animal(0.2);
    println!("Aminal says: {}", animal.speak());
    let animal = get_animal(2.2);
    println!("Aminal says: {}", animal.speak());

    // Adding traits to structs
    let numbers = vec![1, 2, 3];
    println!("Sum of numbers: {}", numbers.sum());


    // Operator overloading
    let p1 = Point { x: 1.0, y: 1.0 };
    let p2 = Point { x: 2.0, y: 2.0 };
    let p3 = p1 + p2;
    println!("{:?}", p3);
}

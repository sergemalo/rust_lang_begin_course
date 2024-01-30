
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
}

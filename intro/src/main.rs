#![allow(unused_variables)]
use std::io;

#[doc = r"Crate comment.
My Hello World app"]




fn main() {

#![doc = r#"# MAIN function
This is the doc of my main
```
fn main()
```

Reads user input and spit it out"#]

    println!("Hello, world!");
    println!("{prenom} {Nom}", prenom="Serge", Nom="Malo");
    // Positionnal + Traits:
    println!("Dec: {0}, Hex: 0x{0:x}", 42);
    // Debug Trait
    println!("DBG: {:?}", [1, 2, 3]);

    // Variables
    // Type is optional 
    // snake_case
    let x = "Serge";
    println!("{}", x);
    // Sometimes, type must be specified to avoid errors
    let amount:i64 = 109_876_543_210;
    // immutable by default
    let mut my_val = 42;
    println!("{}", my_val);
    my_val = 21;
    println!("{}", my_val);
    // Shadowing
    let color = "blue";
    let color = 33;
    println!("{}", color);
    // Declare multiple
    let (a, b, c) = (1, 2, 3);
    println!("{}", a);

    // Scalar
    // Char: size is 4 BYTES !
    let smiley = '\u{1F601}';
    println!("{}", smiley);


    // Strings
    // str vs String:
    // https://dev.to/dsysd_dev/string-vs-str-in-rust-understanding-the-fundamental-differences-for-efficient-programming-4og8#:~:text=In%20summary%2C%20%22String%22%20is,and%20requirements%20of%20your%20code.

    // Next are str or "String slices": they are immutable references (they borrow)
    let dog: &str = "Flash";
    let dog2: &'static str = "Flash-static";
    // String object
    let dog3 = String::new();
    let dog3 = String::from(dog);
    // String format Macro
    let mut owner = String::from(format!("Serge is owner of {}", dog3));
    println!("{}", owner);
    // push_str
    // dog.push_str("---"); !!! Impossible on slices - immutable
    owner.push_str(" with love.");

    // A str is some UTF-8 bytes.
    // A reference to a str (&str) is a pointer to a str plus a length. Knowing the length is obviously important if you actually want to use the pointer.
    // A String is simply one way (albeit the main way) of managing dynamic memory for a str. It's an &str plus the capacity of a memory buffer that the &str can grow into. So in total it's a pointer to a str, the length of the str and the total capacity of the String's buffer.
    // The String can also relocate its buffer to increase its capacity.

    // Constants
    // No "let"
    // Type must be specified
    const SUPER_VAL:&str = "TOTO";
    println!("Super Val:{}", SUPER_VAL);

    // Operations
    // ++ and -- don't exist in Rust !!

    // Functions
    for i in 1..6 {
        // Pass by value (value is borrowed)
        say_something("bla");
    }
    // Pass by Reference - Change where the reference points to
    let mut my_string = "toto";
    replace_string(&mut my_string);
    println!("New String: {}", my_string);
    // Return a value
    println!("---> {}", resturn_a_string());
    // Return a value
    println!("---> {}", resturn_a_str());




    let mut input = String::new();
    println!("Say something:");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You said: {}", input);
        },
        Err(e) => {
            println!("Something went wrong: {}", e);
        }
    }
}



fn say_something(thing: &str) {
    println!("{}", thing);
}


fn replace_string(thing: &mut &str) {
    *thing = "BLA";
}

fn resturn_a_string() -> String {
    let my_string = String::from(format!("This is the returned string"));
    // return keyword can be skipped
    //return my_string;
    my_string

}

// "Static" is required here!!
fn resturn_a_str() -> &'static str {
    let my_string = "This is the returned str";
    // return keyword can be skipped
    //return my_string;
    my_string

}
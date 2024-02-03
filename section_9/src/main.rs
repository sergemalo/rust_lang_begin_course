
// Memory Ownership
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
// Only one variable can OWN a piece of memory
// --> Impossible to have race conditions in parallel execution
// 2 Main ideas:
// 1) For primitive types, copy is cheap.
// 2) For complex types, memory is owned, so ownership has to be transferred


// Borrowing
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
// STILL: Only one variable can OWN a piece of memory
// Variables can borrow ownership from other variables
// IMPORTANT "Mutability" has to be the same from the original variable and the borrowing reference

// Lifetimes
// https://doc.rust-lang.org/book/ch10-02-lifetime-syntax.html
// Rust prevents parts of object outliving the object
// fn get_str() -> &'static str {   // static lifetime means that the string is alive as long as the program
//     "hello"
// }
#[derive(Debug)]
struct Person {
    name: String
}

#[derive(Debug)]
struct Dog<'l>{             // 'l is a lifetime
    name: String,
    owner: &'l Person       // Indicate that Person's lifetime is the same as Dog's (the structure)
}

impl Person {
    // fn get_name(&self) -> &String {
    // EQUIVALENT: 
    fn get_name<'l>(&'l self) -> &'l String {
    // The lifetime 'l indicates that the returned String will live at least as long as self
        &self.name
    }
}

// Reference counted variables
// https://doc.rust-lang.org/book/ch15-03-reference-counted-pointers.html
// A structure that can hold multiple references to the same data
// Note: Rc is not thread safe!!!
// When you clone an Rc, it only increments the reference count rather than creating a deep copy of the data. 
// This can be more efficient than copying the entire data, especially for large or complex data structures. 
// However, it's important to keep in mind that using Rc comes with the overhead of reference counting, so it's best suited for scenarios where the performance impact of reference counting is acceptable.

use std::rc::Rc;

struct Car {
    brand: Rc<String>
}

impl Car {
    fn new(brand: Rc<String>) -> Car {
        Car { brand }
    }
    fn drive(&self) {
        println!("{} is driving", self.brand);
    }
}

fn main() {
    // Ownership
    let i = 5;
    let j = i;      // Copied automically; i and j are now independent and they both still exist
    println!("i = {}, j = {}", i, j);

    let v = vec![1, 2, 3];
    let w = v;          // W has borrowed ownership of v (v is moved), we can't use v anymore.
    // println!("v = {:?}, w = {:?}", v, w); // COMPILE Error
    println!("w = {:?}", w);
    
    // Example 2: Ownership is transfered to a lamba, but the lambda retuns the ownership
    let v2 = vec![1, 2, 3];
    let foo = |v2: Vec<i32> | -> Vec<i32> {
        println!("Using vector in foo");
        v2
    };
    let v2 = foo(v2);
    println!("v2 = {:?}", v2);

    // Example 3: Ownership is transfered to a function
    let v3 = vec![1, 2, 3];
    let v3 = print_size(v3);
    println!("v3 = {:?}", v3);

    // Example 4: When we pass a reference, there is no ownership transfer (no move or copy)
    let v4 = vec![1, 2, 3];
    print_size_ref(&v4);
    println!("v4 = {:?}", v4);

    // Borrowing
    let mut a = 6;
    let b = &mut a;     // b is borrowing memory from a
    println!("b = {}", *b);
    println!("a = {}", a);      // immutable borrowing here - b can't be used anymore
    // println!("b = {}", *b);  // Can't use b anyore, unless we re-declare it (re-borrowing)
    // *b += 2;                 // Can't use b anyore, unless we re-declare it (re-borrowing)
    let b = &mut a;     // b is borrowing memory from a
    *b += 2;
    println!("b = {}", *b);
    println!("a = {}", a);

    let v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i);
        // If we try to change v here, we get a compile error
        // because we can't borrow v as mutable
        //v.push(4);
        // THIS PERFECTLY MAKES SENS! IN C++, this would compile fine but it is probably a bug !!!
    }

    // Lifetimes
    let person = Person { name: String::from("John") };
    let dog = Dog { name: String::from("Fido"), owner: &person };
    println!("Preson name: {}", person.name);
    println!("Dog name: {}", dog.name);
    println!("Dog's owner name: {}", dog.owner.name);

    let mut a: &String = &String::from("No name");
    println!("original name = {}", a);
    {
         let p2 = Person{name: String::from("Serge")};
         println!("p2 name = {}", p2.get_name());
         // a = p2.get_name(); This does not compile - p2 does not live long enough
        a = person.get_name();
    }
    println!("a = {}", a);

    // Reference counted variables
    let brand = Rc::new(String::from("Toyota"));
    println!("My car is a {}", brand);
    println!("Pointers: {} {}", Rc::strong_count(&brand), Rc::weak_count(&brand));
    {
        let car1 = Car::new(brand.clone());
        car1.drive();
        println!("Pointers: {} {}", Rc::strong_count(&brand), Rc::weak_count(&brand));
    }
    println!("My car is a {}", brand);
    
}

fn print_size(v: Vec<i32>) -> Vec<i32> {
    println!("Vector size: {}", v.len());
    v
}

fn print_size_ref(v: &Vec<i32>) -> &Vec<i32> {
    println!("Vector size: {}", v.len());
    v
}
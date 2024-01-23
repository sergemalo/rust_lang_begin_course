
// Global Variables are unsafe
static mut  MY_GLOBAL:i32 = 666;

fn main() {
    
    // FUNCTIONS
    say_hello("Alice");
    say_hello("Bob");
    
    let mut name = "Alice";
    change_name(&mut name);
    say_hello(&name);

    // Scoping	
    {
        let x = 5;
        println!("x = {}", x);
    }
    
    //println!("my_global = {}", MY_GLOBAL);
    // Use unsafe keyword - Not really recommanded...
    unsafe { println!("my_global = {}", MY_GLOBAL); }



    // CLOSURES
    // Closures are anonymous functions that can be passed as arguments to other functions.
    // Closures can capture variables from their environment.
    // When there is no return value, the expression does not have to be surrounded by braces.
    let my_prnt = |x: i32, y: i32| println!("x = {}, y = {}", x, y);
    my_prnt(1, 2);
    let my_add = |x: i32, y: i32| -> i32 { x + y };
    println!("5 + 4 = {}", my_add(5, 4));
    // Closures can be generic
    let my_prnt = |x| println!("x = {}", x);
    // But! You can only use it with one type later on:
    my_prnt("Hello");
    //my_prnt(17);

}

fn say_hello(name: &str) {
    println!("Hello, {}!", name);
}

fn change_name(name: &mut &str) {
    *name = "John";
}
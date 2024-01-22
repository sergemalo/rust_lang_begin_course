
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
}

fn say_hello(name: &str) {
    println!("Hello, {}!", name);
}

fn change_name(name: &mut &str) {
    *name = "John";
}
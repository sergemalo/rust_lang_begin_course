
// Global Variables are unsafe
static mut  MY_GLOBAL:i32 = 666;

// Macros must be defined before we use them
macro_rules! my_macro {
    () => (println!("FIRST MARCO"))
}

macro_rules! name {
    ($name: expr) => (println!("Salut {}", $name))
}

macro_rules! name_star {
    ($($name: expr),*) => {
        $(println!("Salut {}", $name);)*        
        $(println!("{}. t'es dont ben laite", $name);)*        
    };
}

macro_rules! xy {
    (x=> $e: expr) => (println!("x = {}", $e));
    (y=> $e: expr) => (println!("y = {}", $e));
}

// ident is for "identifiers" - Here, the macro writes a function for us.
macro_rules! build_fn {
    ($fn_name: ident) => {
        fn $fn_name() {
            println!("{} called", stringify!($fn_name));
        }
    };
}

fn main() {
    
    // FUNCTIONS
    say_hello("Alice");
    say_hello("Bob");
    
    //let mut name: &str = "Alice";
    let mut name = "Alice";
    change_name(&mut name); // After this call, the string slice "name" points to another string ("John"). No one points to "Alice" anymore.
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



    // Higher Order Functions (HOFs)
    // Functions that take other functions as arguments or return functions as results. 
    let a = 3;
    let b = 5;
    println!("Apply Add: {}", apply(add, a, b));
    println!("Apply Mul: {}", apply(mul, a, b));
    println!("Apply a closure: {}", apply(|x, y| x - y, a, b));

    // Example: calculate the sum of all squares less than 500, only for even numbers
    // 1) W/O HOF
    let limit = 500;
    let mut sum = 0;
    for i in 0.. {
        let isq = i * i;
        if isq > limit {
            break;
        }
        else if is_even(isq) {
            sum += isq;
        }
    }
    println!("Sum = {}", sum);
    // 2) W/ HOF
    let sum = (0..)
        .map(|x| x * x)     // map is a HOF
        .take_while(|&x| x < limit)
        .filter(|&y| is_even(y))
        .fold(0, |sum, x| sum + x);
    println!("Sum = {}", sum);



    // MACROS
    // "Meta programming"
    // Macros are a way to write code that writes code.
    // Examples: println! and debug! and format!
    // macro_rules! my_macro {
    //     (match) => (code to run)
    // }
    my_macro!();
    name!("Bob");
    name_star!("Alice", "Bob", "Charlie");
    xy!(x => 4);
    xy!(y => 5*2);
    build_fn!(hello);   // Write the function
    hello();            // Call the function
}


fn is_even(x: i32) -> bool {
    x % 2 == 0
}

fn say_hello(name: &str) {
    println!("Hello, {}!", name);
}

fn change_name(name: &mut &str) {
    *name = "John";
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn mul(x: i32, y: i32) -> i32 {
    x * y
}   

fn apply(f: fn(i32, i32) -> i32, x: i32, y: i32) -> i32 {
    f(x, y)
}

/* 
fn apply<F>(f: F, x: i32, y: i32) -> i32
    where
        F: Fn(i32, i32) -> i32
{
    f(x, y)
}
*/
use rand::Rng;

fn main() {
    
    // IF
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0, 10);
    println!("X: {}", x);
    if x < 3 {
        println!("< 3");
    } else if x < 6{
        println!("< 6");
    } else {
        println!(">= 6");
    }
    // One-Liner definition
    let y = if x < 5 { true } else { false };
    println!("Y: {}", y);

    // Match
    match x {
        0 => println!("Zero"),
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("Other")
    }       // ===> NO SEMICOLON!
    // Match can return a value (and show how we can use "pattern matching" in that case)
    let y = match x {
        0 => "Zero",
        1 => "One",
        2 | 3 => "Two or Three",
        4..=7 => "Four to Seven",
        _ if (x % 2) == 0 => "Even",
        _ => "Other"
    };        // ===> SEMICOLON!
    println!("Y: {}", y);

    // "Pattern matching"
    let point = (rng.gen_range(0, 2), rng.gen_range(0, 2));
    println!("Point: {:?}", point);
    match point {
        (0, 0) => println!("Origin"),
        (x, 0) => println!("On the X axis at X: {}", x),
        (0, y) => println!("On the Y axis at Y: {}", y),
        (x, y) => println!("Point at ({}, {}) ", x, y)
    }
}

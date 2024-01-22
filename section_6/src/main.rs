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

}

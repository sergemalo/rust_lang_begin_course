

const MY_CONST: i32 = 5;
fn main() {

    // Arrays
    // Size is known at compile time (STATIC)
    // Elements must be of the same type
    // Elements can be accessed by index
    // Elements can be mutated if the array is mutable
    let a = [1, 2, 3, 4, 5];
    let dbls: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    // Using the "Debug" trait to print the contents of an array
    println!("a: {:?}", a);
    println!("dbls: {:?}", dbls);
    // With default value:
    let mut nmbs: [i32; 5] = [MY_CONST; 5]; //Note usage of ";"
    nmbs[0] = 6;
    println!("nmbs: {:?}", nmbs);
    // Using iterators
    for i in nmbs.iter() {
        println!("Value={}", i);
    }

    // Vectors
    // Size is NOT known at compile time (DYNAMIC)
    let mut v: Vec<i32> = Vec::new();
    let mut v2:Vec<i32> = vec![1, 2, 3];
    v.push(17);
    println!("v: {:?}", v);
    v2.push(11);
    v2.remove(2);
    println!("v2: {:?}", v2);
    let v3:Vec<i32> = vec![10; 2];
    println!("v3: {:?}", v3);
    for i in v3.iter() {
        println!("Value={}", i);
    }

    // Slices

}


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

    let mut v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i);
        // If we try to change v here, we get a compile error
        // because we can't borrow v as mutable
        //v.push(4);
        // THIS PERFECTLY MAKES SENS! IN C++, this would compile fine but it is probably a bug !!!
    }

    
}

fn print_size(v: Vec<i32>) -> Vec<i32> {
    println!("Vector size: {}", v.len());
    v
}

fn print_size_ref(v: &Vec<i32>) -> &Vec<i32> {
    println!("Vector size: {}", v.len());
    v
}
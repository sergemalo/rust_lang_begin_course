
// Defining my own class
// I can add the "Clone" trait to be able to clone it
// When I want to pass this structure by value, I must clone it before
// "Copy" trait cannot be implemented automatically
#[derive(Debug, Clone)]
struct MyBigStruct {
    first_name:     String,
    last_name:      String,
    phone_number:   String,
    age:            u8
}

fn main() {
    println!("Hello, world!");


    // PASSING BY VALUE
    println!("---- Pass a CONSTANT value -----");
    let value: i32 = 17_000_000;
    println!("My Value {} is at address: 0x{:p}", value, &value);
    show_value(value);
    show_value_i32(value);
    let my_string = String::from(format!("Hello World!"));
    println!("My String {} is at address: 0x{:p}", my_string, &my_string);
    show_value(my_string.clone());          // String must be cloned - Copy trait not implemented
    show_value_string(my_string.clone());   // String must be cloned - Copy trait not implemented
    let my_big = MyBigStruct {
        first_name: "Serge".to_string(),
        last_name: "Malo".to_string(),
        phone_number: "514-555-5678".to_string(), 
        age: 48
    };
    println!("My Struct {:?} is at address: 0x{:p}", my_big, &my_big);
    show_value(my_big.clone());             // My Struct must be cloned - Copy trait not implemented
    show_value_struct(my_big.clone());      // My Struct must be cloned - Copy trait not implemented

    println!("---- Pass a mutable value -----");
    // Nothing is different from the previous section
    let mut value: i32 = 17_000_001;
    println!("My Value {} is at address: 0x{:p}", value, & value);
    show_value(value);
    show_value_i32(value);
    value = 17_000_002;
    println!("My Value {} is at address: 0x{:p}", value, & value);
    show_mut_value(value);
    println!("My Value {} is at address: 0x{:p}", value, & value);

    /////////////////////////////////////////////////////
    println!("---- Pass by reference to value -----");
    let value: i32 = 17_000_003;
    let ref_value = & value;
    println!("My Value {} is at address: 0x{:p}", value, & value);
    println!("My Reference to Value {} points to: 0x{:p}", ref_value, ref_value);
    println!("My Reference to Value {} is at address: 0x{:p}", ref_value, & ref_value);
    show_ref_value_i32(&value);
    show_ref_value(ref_value);
    let ref_value = & my_string;
    println!("My String {} is at address: 0x{:p}", my_string, & my_string);
    println!("My Reference to String {} points to: 0x{:p}", ref_value, ref_value);
    println!("My Reference to String {} is at address: 0x{:p}", ref_value, & ref_value);
    show_ref_value_string(&my_string);
    show_ref_value(ref_value);
    let ref_value = & my_big;
    println!("My Big {:?} is at address: 0x{:p}", my_big, & my_big);
    println!("My Reference to Big {:?} points to: 0x{:p}", ref_value, ref_value);
    println!("My Reference to Big {:?} is at address: 0x{:p}", ref_value, & ref_value);
    show_ref_value_struct(&my_big);
    show_ref_value(ref_value);

    println!("---- Pass by reference to mutable value -----");
    let mut value: i32 = 17_000_004;
    println!("My Value {} is at address: 0x{:p}", value, & value);
    change_ref_to_mut_value(& mut value); // NOTE: "mut" must be added, otherwise it will be a reference to a non-mutable value
    println!("My Value {} is at address: 0x{:p}", value, & value);

    let mut value: i32 = 17_000_005;
    let ref_value = & mut value;
    //println!("My Value {} is at address: 0x{:p}", value, & value); // Can't use this, because it will create a 2nd reference to the same mutable value
    println!("My Reference to Value {} points to: 0x{:p}", ref_value, ref_value);
    println!("My Reference to Value {} is at address: 0x{:p}", ref_value, & ref_value);
    change_ref_to_mut_value(ref_value);
    println!("My Reference to Value {} points to: 0x{:p}", ref_value, ref_value);


    // In Rust, it is generally better to pass a mutable reference to a variable if you want to modify the variable within a function. This allows the function to directly modify the original variable without taking ownership of it.
    // Passing a reference to a mutable variable would require dereferencing the reference to modify the original variable, which can be less ergonomic and introduce additional overhead. Using a mutable reference directly allows for clearer and more efficient code.

    // Pass by mutable reference to a value
    let value: i32 = 17_000_006;
    let ref_value = & value;
    println!("My Value {} is at address: 0x{:p}", value, & value);
    println!("My Mutable Reference to Value {} points to: 0x{:p}", ref_value, ref_value);
    println!("My Mutable Reference to Value {} is at address: 0x{:p}", ref_value, & ref_value);
    change_mut_ref_to_value(ref_value);
    println!("My Reference to Value {} points to: 0x{:p}", ref_value, ref_value);


    //change_value(&value);
    //println!("My Value now: {}", value);

}

////////////////////////////////////////
// Pass by value
fn show_value_i32(value: i32) {
    println!("  show_value_i32 - Input value {} is at address: 0x{:p}", value, &value);
}

fn show_value_string(value: String) {
    println!("  show_value_string - Input value {} is at address: 0x{:p}", value, &value);
}

fn show_value_struct(value: MyBigStruct) {
    println!("  show_value_struct - Input value {:?} is at address: 0x{:p}", value, &value);
}

fn show_value<T: std::fmt::Debug>(value: T) {
    println!("  show_value<T> - Input value {:?} is at address: 0x{:p}", value, &value);
}

// Pass a mutable value
// Variable used to call the function is NOT modified.
// It is only useful if we want to modify it in the function.
fn show_mut_value(mut m_value: i32) {
    println!("  show_mut_value - Input value {} is at address: 0x{:p}", m_value, &m_value);
    m_value = 18;
    println!("  show_mut_value - After modification, Input value {} is at address: 0x{:p}", m_value, &m_value);
}

//////////////////////////////////////
// Pass by reference to value
 fn show_ref_value_i32(ref_value: &i32) {
    println!("  show_ref_value - Input value {} points to {:p} is at address: 0x{:p}", ref_value, ref_value, &ref_value);
}

fn show_ref_value_string(ref_value: &String) {
    println!("  show_value_string - Input value {} points to {:p}  is at address: 0x{:p}", ref_value, ref_value, &ref_value);
}

fn show_ref_value_struct(ref_value: &MyBigStruct) {
    println!("  show_value_struct - Input value {:?} points to {:p}  is at address: 0x{:p}", ref_value, ref_value, &ref_value);
}

fn show_ref_value<T: std::fmt::Debug>(ref_value: &T) {
    println!("  show_value<T> - Input value {:?} points to {:p}  is at address: 0x{:p}", ref_value, ref_value, &ref_value);
}


// Pass by reference to mutable value
fn change_ref_to_mut_value(ref_to_m_value: &mut i32) {
    println!("  change_ref_to_mut_value - Input value {} is at address: 0x{:p}", ref_to_m_value, &ref_to_m_value);
    *ref_to_m_value = 19;
    println!("  change_ref_to_mut_value - After modification, Input value {} is at address: 0x{:p}", ref_to_m_value, &ref_to_m_value);
}

// Pass by mutable reference to value ===> THE REFERENCE IS PASSED BY COPY
fn change_mut_ref_to_value(mut mut_ref_to_value: &i32) {
    println!("  change_mut_ref_to_value - Input value {} is at address: 0x{:p}", mut_ref_to_value, &mut_ref_to_value);
    mut_ref_to_value = &20;
    println!("  change_mut_ref_to_value - After modification, Input value {} is at address: 0x{:p}", mut_ref_to_value, &mut_ref_to_value);
}

/*
fn change_value(mut value: &i32) {
    println!("  Input value: {}", value);
    *value = 18;
    println!("  Input value changed: {}", value);
}
*/

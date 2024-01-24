fn main() {
    println!("Hello, world!");

    // Pass by value
    let value: i32 = 17_000_000;
    let addr = & value;
    println!("My Value {} is at address: 0x{:p}", value, addr);
    show_value(value);

    let mut value: i32 = 17_000_001;
    println!("My Value {} is at address: 0x{:p}", value, & value);
    show_value(value);
    value = 17_000_002;
    println!("My Value {} is at address: 0x{:p}", value, & value);

    show_mut_value(value);
    println!("My Value {} is at address: 0x{:p}", value, & value);


    // Pass by reference to value
    let value: i32 = 17_000_003;
    let value_ref = & value;
    println!("My Value {} is at address: 0x{:p}", value, & value);
    println!("My Reference to Value {} points to: 0x{:p}", value_ref, value_ref);
    println!("My Reference to Value {} is at address: 0x{:p}", value_ref, & value_ref);
    show_ref_value(&value);
    show_ref_value(value_ref);

    // Pass by reference to mutable value
    let mut value: i32 = 17_000_004;
    println!("My Value {} is at address: 0x{:p}", value, & value);
    change_mut_ref_value(& mut value); // NOTE: "mut" must be added, otherwise it will be a reference to a non-mutable value
    println!("My Value {} is at address: 0x{:p}", value, & value);

    let mut value: i32 = 17_000_005;
    let value_ref = & mut value;
    //println!("My Value {} is at address: 0x{:p}", value, & value); // Can't use this, because it will create a 2nd reference to the same mutable value
    println!("My Reference to Value {} points to: 0x{:p}", value_ref, value_ref);
    println!("My Reference to Value {} is at address: 0x{:p}", value_ref, & value_ref);
    change_mut_ref_value(value_ref);
    println!("My Reference to Value {} points to: 0x{:p}", value_ref, value_ref);


    //change_value(&value);
    //println!("My Value now: {}", value);

}

////////////////////////////////////////
// Pass by value
fn show_value(value: i32) {
    println!("  show_value - Input value {} is at address: 0x{:p}", value, &value);
}

// Pass a mutable value
// Variable used to call the function is NOT modified.
// It is only useful if we want to modify it in the function.
fn show_mut_value(mut value: i32) {
    println!("  show_mut_value - Input value {} is at address: 0x{:p}", value, &value);
    value = 18;
    println!("  show_mut_value - After modification, Input value {} is at address: 0x{:p}", value, &value);
}

//////////////////////////////////////
// Pass by reference
 fn show_ref_value(value: &i32) {
    println!("  show_ref_value - Input value {} is at address: 0x{:p}", value, &value);
}

// Pass by reference to mutable value
fn change_mut_ref_value(value: &mut i32) {
    println!("  change_mut_ref_value - Input value {} is at address: 0x{:p}", value, &value);
    *value = 18;
    println!("  change_mut_ref_value - After modification, Input value {} is at address: 0x{:p}", value, &value);
}


/*
fn change_value(mut value: &i32) {
    println!("  Input value: {}", value);
    *value = 18;
    println!("  Input value changed: {}", value);
}
*/

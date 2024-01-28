fn main() {

    // Rules for me:
    // Use constant string slices (str&) as much as possible
    // Use String when you need to use the string as a buffer that can have varying size

    //let mut a:&str = "ZZZ";
    //let b:&mut &str = &mut a;
    //a = "AAA";
    //println!("a: {}, b: {}", a, b);
    //*b = "BBB";
    //println!("a: {}, b: {}", a, b);




    // &str is a "string slice"
    let a_string_slice:&str = "=== this is a string slice ===";
    let a_copy_of_a_string_slice:&str = a_string_slice;
    let a_string:String = String::from("*** this is a string. ***");
    let a_string_ref:&String = &a_string;

    println!("a_str: {}, a_str_ref: {}, a_string: {}, a_string_ref: {}", a_string_slice, a_copy_of_a_string_slice, a_string, a_string_ref);

    print_a_str(a_string_slice);
    print_a_str(&a_string_slice);
    print_a_str(a_copy_of_a_string_slice);
    print_a_str(a_string.as_str());
    print_a_str(&a_string);
    print_a_str(a_string_ref.as_str());
    print_a_str((*a_string_ref).as_str());

    //print_a_string(&a_str);
    //print_a_string(a_str_ref);
    //print_a_string(a_string.as_str());
    print_a_string(&a_string);
    //print_a_string(a_string_ref.as_str());
    //print_a_string((*a_string_ref).as_str());



    let mut a_mut_str:&str = "___ this is a mut string slice ___";
    println!("My Reference to Value {} points to: 0x{:p}", a_mut_str, a_mut_str);
    change_a_mut_str(&mut a_mut_str);
    println!("My Reference to Value {} points to: 0x{:p}", a_mut_str, a_mut_str);
    print_a_str(&a_mut_str);

    //let a_mut_str2:& &str = "___ this is a mut string slice 2 ___";
    //change_a_mut_str(&mut a_mut_str2);
    //print_a_str(&a_mut_str2);

    //let a_mut_str_ref = &mut a_str;
    //change_a_mut_str(a_mut_str_ref);
    //print_a_str(&a_mut_str_ref);

    //let mut a_mut_string = String::from("$$$ this is a mut string. $$$");
    //change_a_mut_str(a_mut_string.as_mut_str());
    //print_a_str(&a_mut_string);
    //let a_mut_string_ref = &mut a_string;
    //change_a_mut_str(a_mut_str_ref);
    //print_a_str(&a_mut_str_ref);

}


fn print_a_str(a_str: &str) {
    println!("Function print_a_str: {}", a_str);
}

fn print_a_string(a_string: &String) {
    println!("Function print_a_string: {}", a_string);
}

fn change_a_mut_str(a_mut_str: &mut &str) {
    *a_mut_str = ">>> changed str <<<";
}






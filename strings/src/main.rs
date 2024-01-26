fn main() {

    let a_str = "=== this is a str ===";
    let a_str_ref = &a_str;
    let a_string = String::from("*** this is a string. ***");
    let a_string_ref = &a_string;

    println!("a_str: {}, a_str_ref: {}, a_string: {}, a_string_ref: {}", a_str, a_str_ref, a_string, a_string_ref);

    print_a_str(&a_str);
    print_a_str(a_str_ref);
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
}


fn print_a_str(a_str: &str) {
    println!("Function print_a_str: {}", a_str);
}

fn print_a_string(a_string: &String) {
    println!("Function print_a_string: {}", a_string);
}





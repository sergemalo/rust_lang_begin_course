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

    // A reference to a contiguous sequence of elements in a vector
    let v = vec![1, 2, 3, 4, 5];
    let slice = &v[1..3];
    println!("slice: {:?}", slice);
    // Size of slice is know at runtime
    println!("size of slice: {}", slice.len());
    // Can be used on arrays, vectors and Strings
    // Are indexed

    // Slices TO MUTABLE ARRAYS
    // Slices to mutable arrays allow us to change elements - not the size
    // First define a mutable array
    let mut colors = ["red", "green", "blue"];
    // As a mutable array, we can change the elements.
    colors[1] = "yellow";
    println!("colors: {:?}", colors);
    // 
    // Define a slice to that mutable array
    // Note that the slice itself is not mutable - it is a reference to a mutable array
    let slice_to_arr = &mut colors[1..3];
    slice_to_arr[1] = "orange";
    println!("colors: {:?}", colors); // AFTER THIS CALL, We can't use the slice slice_to_arr anylonger
    // Do the same, with a function
    // 3 different ways to do that:
    update_colors(&mut colors[1..3]); // cast to a slice to mutable array
    println!("colors: {:?}", colors);
    colors[2] = "orange";
    update_colors(colors[1..3].as_mut()); // use as_mut()
    println!("colors: {:?}", colors);
    colors[2] = "orange";
    let slice_to_arr2 = &mut colors[1..3];
    update_colors(slice_to_arr2); // use the defined slice
    println!("colors: {:?}", colors);


    // Tuples
    // They are STATIC, but values can be changed
    // Indexed with the "." operator
    // LIMIT OF 12 ELEMENTS !!!
    let mut person = ("John", "Doe", 32);
    println!("person: {:?}", person);
    person.2 = 33;
    print!("person: {}, {}, {}", person.0, person.1, person.2);
    let (first, last, age) = person;
    println!("first: {}, last: {}, age: {}", first, last, age);
    
    // Structures
    // Key/Value pair OR Class
    let emp1 = Employee {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        dept: "Engineering".to_string(),
        age: 32
    };
    println!("emp1: {:?}", emp1);
    println!("emp1: {}, {}, {}, {}", emp1.first_name, emp1.last_name, emp1.dept, emp1.age);
    println!("emp1: {}", emp1.full_name());
    println!("emp1: {}", Employee::default_name());

    // Enums
    // Data types can be different
    let c = Color::BrunCaca;
    println!("c: {:?}", c);
    match c {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
        Color::BrunCaca => println!("BRUN CACA")
    }
    

}

fn update_colors(colors_slice: &mut [&str]) {
    colors_slice[1] = "brown";
}

#[derive(Debug)]    // Add the "Debug" trait to print the contents of a struct
struct Employee {
    first_name: String,
    last_name: String,
    dept: String,
    age: u8
}

impl Employee {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Static method: does use the self parameter
    fn default_name() -> String {
        "Gros Tas".to_string()
    }
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    BrunCaca
}



// 2 types of error: 
// Unrecoverable: panic!
// Recoverable: return enum

// panic! macro stops the execution with a message

// return enum


// Error handling with files
use std::fs::{File, OpenOptions, remove_file};
use std::io::Write;
use std::io::Read;


// Error handling Helper methods
// unwrap

fn read_user_name_from_file() -> Result<String, std::io::Error> {
    let f = File::open("username.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_user_name_from_file2() -> Result<String, std::io::Error> {
    let mut f = File::open("username.txt")?;            // ? operator   Same as unwrap()
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    
    // PANIC example
    // let v = vec![1, 2, 3];
    // v[99];  -- this will panic
    // PANIC example 2
    // panic!("crash and burn");

    // matching on Result enum
    /*
    let f = File::open("cacacaca.txt");
    match f {
        Ok(file) => {
            println!("File found:\n{:?}", file);
        }
        Err(error) => {
            panic!("Problem opening the file:\n{:?}", error)
        },
    };
    */

    divide(Some(10));
    divide(Some(100));
    divide(None);
    //divide(Some(0)); This will panic
    
    
    // Create write-only file
    let mut f = File::create("hello.txt").expect("Creating a file failed");
    f.write_all("HELLO WORLD\n".as_bytes()).expect("Writing to a file failed");

    // Open the file to append to it
    let mut f = OpenOptions::new().append(true)
        .open("hello.txt")
        .expect("Opening a file failed");
    f.write_all("Appending data...".as_bytes()).expect("Writing to a file failed");

    // Open the file in read-only mode
    let mut f = File::open("hello.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    println!("Read this for the file:\n{}", s);

    // Delete a file
    remove_file("hello.txt").expect("Deleting a file failed");


    // unwrap example
    //let _f = File::open("cacacaca.txt").unwrap();
    //let _f = File::open("cacacaca.txt").expect("Ton fichier n'existe pas");
    /*
    let f = File::open("cacacaca.txt");
    match f {
        Ok(file) => {
            println!("File found:\n{:?}", file);
        }
        Err(error) => {
            panic!("Problem opening the file:\n{:?}", error)
        },
    };
    */
        


    // ? Operator
    // When we don't care about the error
    let a = read_user_name_from_file();
    println!("User name: {:?}", a);

    let a = read_user_name_from_file2();
    println!("User name: {:?}", a);


}

const REPONSE_UNI: i32 = 42;

fn divide(x: Option<i32>) -> i32 {
    let my_result: i32;
    match x {
        Some(0) => panic!("Dividing by zero"),
        Some(x) => {
            my_result = x/REPONSE_UNI;
            println!("Your value divided by {} is {}", x, my_result);
        }
        None => {
            println!("No value provided");
            my_result = REPONSE_UNI;
        }
    }
    println!("Result: {}", my_result);
    my_result
}

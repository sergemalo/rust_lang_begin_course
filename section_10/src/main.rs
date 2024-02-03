

// Error handling with files
use std::fs::{File, OpenOptions, remove_file};
use std::io::Write;
use std::io::Read;


fn main() {
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

}

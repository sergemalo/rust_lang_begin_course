use std::thread;
use std::time::Duration;
use std::thread::sleep;
// Rust ownership/borrowing:
// * Memory safety
// * No Data races

fn main() {
    let mut threads = vec![];

    for i in 0..10 {
        let th = thread::spawn(move || {
            println!("New thread: {}", i);
            sleep(Duration::from_secs(1));
            println!("New thread: {} DONE!", i);
        });
        threads.push(th);
    }

    // Waiting for all threads to finsih: calling join
    for t in threads {
        t.join().unwrap();
    }
    println!("Hello, world!");
}

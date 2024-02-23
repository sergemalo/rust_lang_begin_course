use std::thread;
use std::time::Duration;
use std::thread::sleep;
use std::sync::mpsc;
// Rust ownership/borrowing:
// * Memory safety
// * No Data races

// Channels:
// a Way to send data between threads
// MPSC:  Multiple producer, single consumer

const NB_THREADS: usize = 5; // usize is used for calling thread methods (local system dependent)

fn start_thread(d: usize, tx: mpsc::Sender<usize>) {
    thread::spawn(move || {
        println!("Seeting timer {}", d);
        sleep(Duration::from_secs(d as u64));
        println!("Sending the message {}", d);
        tx.send(d).unwrap();
    });
}

fn main() {

    main_for_threads();

    basic_channel_example();

    let (tx, rx) = mpsc::channel();
    for i in 0..NB_THREADS {
        let tx = tx.clone();
        start_thread(i, tx);
    }
    // IMPORTANT: When you use rx.iter() without take(NB_THREADS), 
    // the iter() method creates an iterator over the elements of the channel, 
    // and since the channel is asynchronous, it will keep waiting for new elements to arrive indefinitely. 
    // This can lead to an infinite loop if the channel remains open and continues to receive new elements without a limit on the iteration.    
    for i in rx.iter().take(NB_THREADS) {       // Without "take" it would be infinite
        println!("Received: {}", i);
    }
}

fn basic_channel_example() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let msg = String::from("Hello from thread");
        tx.send(msg).unwrap();
    });
    let msg = rx.recv().unwrap(); // recv is a blocking call
    println!("Received: {}", msg);
    
}

fn main_for_threads() {
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
    println!("Main thread completes!");
}

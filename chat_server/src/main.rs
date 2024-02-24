use std::io::{ErrorKind,Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32; // Characters

fn main() {

    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server.set_nonblocking(true).expect("Failed to initialize non-blocking");

    let mut clients = vec![];
    // Create Tx/Rx channels: the messages received from the clients will be pushed in this main loop,
    // such that messages are printed in a single thread (the main thread).
    let (tx, rx) = mpsc::channel::<String>();

    loop {
        // The accept method returns a tuple when a client is ready to be connected.
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);

            // Now, we have a new client connected
            // We need to create a thread, to handle seperately all the clients
            // The thread is a simple non-blocking loop listening for messages
            let tx = tx.clone();
            clients.push(socket.try_clone().expect("Failed to clone client"));
            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];
                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");

                        println!("{}: {:?}", addr, msg);
                        tx.send(msg).expect("Failed to send msg");

                    },
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("Closing connection with: {}", addr);
                        break;
                    }
                }

                // Sleep for a bit to prevent busy looping;
                // We check for a new message every 100ms
                sleep();

            });
        }

        // Check for received messages from any thread handling the clients
        if let Ok(msg) = rx.try_recv() {
            clients = clients.into_iter().filter_map(|mut client| {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).map(|_| client).ok()
            }).collect::<Vec<_>>();
        }

        // Sleep for a bit to prevent busy looping
        sleep();
    }
}

fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}
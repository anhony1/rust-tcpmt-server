use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use rand::Rng;


fn handle_client(stream: TcpStream) {

    // Start a new thread for every time handle_client get's called. 

    thread::spawn(|| {

        println!("Handling Stream :) ");

        let mut rng = rand::thread_rng();

        let n1: u8 = rng.gen();
        let n2: u64 = rng.gen();

        for i in 1..10 {

            println!("Processing number: {}", n1);
            thread::sleep(Duration::from_millis(10));

        }

        println!("Thread Finished");

    });

}

pub fn start() {

    println!("Starting Server...");

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    
    for stream in listener.incoming() {
        
        match stream {

            Ok(s) => {
                handle_client(s);
            }
            Err(e) => {
                eprintln!("Error Encountered: {}", e);
            }
 
        }

    }

}


pub fn stop() {

    // Stop all threads GRACEFULLY?

    println!("Stopping Server...");

}
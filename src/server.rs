use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use rand::Rng;
use std::sync;

struct Worker { 

    id: usize,
    thread: Option<thread::JoinHandle<()>>,

}

impl Worker {
    fn new(
        id: usize,
        job_queue: Arc<SegQueue<Job>>,
        job_signal: Arc<(Mutex<bool>, Condvar)>,
        running: Arc<AtomicBool>,
    ) -> Worker {
        let thread = thread::spawn(move || loop {
            match job_queue.pop() {
                Some(Job::Task(task)) => if let Err(_) = task() {},
                Some(Job::Shutdown) => {
                    break;
                }
                None => {
                    let (lock, cvar) = &*job_signal;
                    let mut job_available = lock.lock().unwrap();
                    while !*job_available && running.load(Ordering::Relaxed) {
                        job_available = cvar
                            .wait_timeout(job_available, Duration::from_millis(100))
                            .unwrap()
                            .0;
                    }
                    *job_available = false;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}


fn handle_client(stream: TcpStream) {

    // Start a new thread for every time handle_client get's called. 

    thread::spawn(|| {

        println!("Handling Stream...");

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
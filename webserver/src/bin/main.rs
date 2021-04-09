use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};
use webserver::ThreadPool;
use std::sync::{Arc, Mutex};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    let keep_running = Arc::new(Mutex::new(true));
    let check_keep_running = Arc::clone(&keep_running);

    let stream_listener = thread::spawn(move || {
        for stream in listener.incoming() {
            let stream = stream.unwrap();

            pool.execute(|| {
                handle_conenction(stream);
            });
            println!("Checking for termination message from terminal... ");
            if ! *check_keep_running.lock().unwrap(){
                break;
            }
        }
    });

    loop {
        println!("Enter a command for web server: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Enter a valid string");

        if input.trim() == "end" {
            * keep_running.lock().unwrap() = false;
            break;
        } else {
            println!("Enter 'end' to stop execution");
            continue;
        }
    }

    stream_listener.join().unwrap();

    println!("Shutting down.")
}

fn handle_conenction(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "final.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "final.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

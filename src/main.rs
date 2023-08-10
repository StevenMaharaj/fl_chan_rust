#![warn(dead_code)]
use std::{
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let shared_data = Arc::new(Mutex::new(0));

    let mut handles = vec![];
    let shared_data1 = Arc::clone(&shared_data);
    let handle = thread::spawn(move || {
        producer2(shared_data1);
    });
    handles.push(handle);

    let listener = std::net::TcpListener::bind("localhost:6900").unwrap();
    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
        let shared_data = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            consumertcp(shared_data, stream);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    fn producer2(shared_data: Arc<Mutex<u8>>) {
        let mut x = 0;
        loop {
            thread::sleep(Duration::from_millis(1000));
            x += 1;
            // println!("Producer: {}", x);
            let mut data = shared_data.lock().unwrap();
            *data = x % 255;
        }
    }

    fn consumertcp(shared_data: Arc<Mutex<u8>>, mut stream: TcpStream) {
        let mut last = 0;
        loop {
            // std::io::ErrorKind::Interrupted;
            thread::sleep(Duration::from_millis(1));
            let data = shared_data.lock().unwrap();

            if *data > last {
                match stream.write_all(&data.to_be_bytes()) {
                    Ok(_) => {
                        stream.flush().unwrap();
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        break;
                    }
                }
                last = *data;
            }
        }
    }
}

use std::{ io::{IoSlice, Read, Write}, iter::StepBy, net::TcpListener, sync::mpsc, thread::{self, sleep}, time::Duration};


// struct Worker{
//     id:usize,
//     thread: thread::JoinHandle<()>
// }
// pub struct ThreadPool {
//     threads: Vec<Worker>,
//     sender: mpsc::Sender<Job>,
// }

// impl Worker{
//     pub fn new(id:usize,receiver: mpsc::Receiver<Job>) -> Worker {
//         Worker {
//             id,
//             thread: thread::spawn(move ||{
//                 receiver;
//             })
//         }
//     }
// }

// impl ThreadPool {
//     pub fn new(size: usize) -> ThreadPool {
//         assert!(size > 0);

//         let mut threads = Vec::with_capacity(size);

//         for id in 0..size {
//             let worker = Worker::new(id);
//             threads.push(worker);
//         }

//         ThreadPool {
//             threads
//         }
//     }
// }
fn main() {
    let listner = TcpListener::bind("127.0.0.1:8080").unwrap();

    for mut stream in listner.incoming(){

        thread::spawn(move ||{
        match stream {
            Ok(mut stream)=>{

                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();

                String::from_utf8_lossy(&buffer);
                println!("Request : {:?}",String::from_utf8_lossy(&buffer[..]));

                println!("Some : {:?}",stream);
                let msg = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 13\r\n\r\nHello, World!";
                
                stream.write_all(msg.as_bytes()).unwrap();
                sleep(Duration::from_secs(10));

                stream.write_timeout().unwrap();
                
                stream.flush().unwrap()
                
            }
            Err(e)=> println!("Error : {:?}",e)
        }
        }
    );
       
    }
}

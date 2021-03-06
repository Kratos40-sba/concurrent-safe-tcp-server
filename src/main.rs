use std::{io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex}};
use std::collections::VecDeque;
#[derive( Debug , Eq , PartialEq)]
enum Request {
    Publish(String),
    Retrive,
}


fn read_line(stream: &TcpStream) -> String {
    let mut buffer_reader = BufReader::new(stream);
    let mut buf = String::new();
    buffer_reader.read_line(&mut buf).unwrap();
    buf
}
fn parse_request(line: String) -> Request{
  let trimmed = line.trim_end(); 
  if trimmed.is_empty() {
       Request::Retrive
     } else {
       Request::Publish(String::from(trimmed))
  }
  
}
fn handle_client(mut stream: TcpStream , storage : &Mutex<VecDeque<String>>) -> () {
    let line = read_line(&stream);
    let req = parse_request(line);
    match  req{
        Request::Publish(msg) => {
                let  mut guard = storage.lock().unwrap();
                guard.push_back(msg);    
                drop(guard)        
        }
        Request::Retrive =>{
            let maybe_msg = storage.lock().unwrap().pop_front();
            match maybe_msg {
                Some(msg) => {
                    stream.write_all(msg.as_bytes()).unwrap();
                    
                }
                None => {
                    stream.write_all("NO MESSAGE !! \n".as_bytes()).unwrap();
                }
            }
        }
    }
}
fn main() {
    let l = TcpListener::bind("127.0.0.1:7878").unwrap();
    let  storage = Arc::new(Mutex::new( VecDeque::new()));
    for connection_attempt in l.incoming() {
        match connection_attempt {
            Ok(stream) => {
                let thread_handle = Arc::clone(&storage);
                 // storage.clone() (less performant)
                std::thread::spawn(move  || {
                    handle_client(stream ,  &thread_handle);
                });               
            },
            Err(e) => {
                eprintln!("Error connecting : {}", e)
            }
        }
    }
}

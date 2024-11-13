use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
   let listener: TcpListener = TcpListener::bind("0.0.0.0:3000").unwrap();

   for stream   in listener.incoming() {
    handle_client(stream.unwrap());
   }
}

fn handle_client(mut stream: TcpStream) {
   let mut buffer = [0; 1024];
   stream.read(&mut buffer).unwrap();
   println!("Request: {}", String::from_utf8_lossy(&buffer));

   let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";
   stream.write(response.as_bytes()).unwrap();
   stream.flush().unwrap();
}
 
//  The code above is a simple HTTP server that listens on port 3000 and responds with a simple message. 
//  The  handle_client  function reads the incoming request and prints it to the console. It then sends a response back to the client. 
//  The  main  function binds the server to the address


use std::net::{TcpStream};
use std::io::{Read,Write};
//use std::str::from_utf8;


fn main() {
    
    match TcpStream::connect("localhost:6666") {

        Ok(mut stream) => {

            println!(".> Successful connection");
            
            loop {
                //set a 50 byte buffer
                let mut buffer = [0 as u8; 50];

                match stream.read(&mut buffer) {
                    Ok(size) if size > 0 => {

                        let received = &buffer[0..size]; // slice the buffer to the actual received size.

                        match String::from_utf8(received.to_vec()) {
                           Ok(command) => {

                                println!(".> Recieved command: {}", command.trim());

                                //process command here using OS commands

                                let msg = b"DEBUG stream write DEBUG";

                                stream.write(msg).unwrap();
                           } 
                           Err(_) => {
                                println!("!> Received non-UTF-8 data");
                            }
                        }

                        //println!(".> Received command: {:?}", received);
                    }
                    Ok(_) => {
                        println!("!> Server closed the connection.");
                        break; // exit loop if the server closes the connection
                    }
                    Err(e) => {
                        println!("!> Error reading from the server: {}", e);
                    }

                }

            }


        }
        Err(e) => {
            println!("!> Failed to connect: {}", e);
        }

    }
    println!("!> Terminated");

 }
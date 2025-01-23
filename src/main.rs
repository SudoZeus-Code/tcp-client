
use std::net::{TcpStream};
use std::io::{Read,Write};
use std::str::from_utf8;


fn main() {
    
    match TcpStream::connect("localhost:6666") {
        Ok(mut stream) => {
            println!(".> Successful connection");
            let msg = b"Client Hello...";

            stream.write(msg).unwrap();

            println!(".> Sent message: '{}' awaiting reply...", from_utf8(msg).unwrap());

            // using a 16 byte buffer, changed from 6, why? no idea
            //this changed becuase of line 12, the bytes sent.

            //INcreased buffer size to 50 bytes
            let mut buffer = [0 as u8; 50];

            //changed stream.read_exact to stream.read because read_exact requires the server to send data of exactly the specificed length, which isnt garueneed. stream.read will handle variable-lengh responses. 
            match stream.read(&mut buffer) {
                Ok(size) if size > 0 => {

                    // This block runs if the read was successful and more than 0 bytes were received

                    // improper response check, The comparison if &data == msg checks for a byte-for-byte match but may fail if the server sends a slightly different response or includes padding.
                    
                    // slice the actual data recieved.
                    let recieved_bytes = &buffer[0..size];
                    println!(".> Raw bytes received: {:?}", recieved_bytes);

                    match from_utf8(recieved_bytes) {
                        Ok(text) => {
                            println!(".> Reply from server: '{}'", text.trim());
                        }
                        Err(_) => {
                            println!("!> Recieved non-UTF8 data");
                        }

                    }

                }
                Ok(_) => {
                    println!("!> Server closed the connection without sending data.");
                }
                Err(e) => {
                    println!("!> Failed to read data: {}", e );
                }


            } 

        }
        Err(e) => {
            println!("!> Failed to connect: {}", e);
        }

    }
    println!("!> Terminated");

 }
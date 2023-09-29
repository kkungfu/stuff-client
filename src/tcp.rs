use std::io;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

fn print(bytes: &[u8]) {
    match std::str::from_utf8(bytes) {
        Ok(string) => { println!("PRINT {}", string); }
        Err(_) => { println!("PRINT ERROR") }
    }
}

pub fn tcp_listen(mut stream: TcpStream) {
    let mut receive_buffer = [0; 4098];

    loop {
        match stream.read(&mut receive_buffer) {
            Ok(received_size) => {
                if received_size == 0 {
                    return
                }

                let received_data = &receive_buffer[0..received_size];
                match stream.write(received_data) {
                    Ok(send_size) => {
                        if send_size != received_size {
                            println!("** STREAM RESEND ERROR **");
                            return
                        }

                        println!("** STREAM PING PONG **");
                        match received_data {
                            b"@command logs" => {
                                println!("server is asking logs");
                            },
                            _ => {}
                        }
                        print(received_data);
                    }
                    Err(_) => {
                        println!("** STREAM STOPPED (WRITE) **");
                        return
                    }
                }
            }
            Err(_) => {
                println!("** STREAM STOPPED (READ) **");
                return
            }
        }
    }
}

pub fn tcp_client(mut stream: TcpStream) {
    let mut send_buffer = String::new();
    let mut receive_buffer = [0; 4098];

    loop {
        send_buffer.clear();
        io::stdin().read_line(&mut send_buffer).unwrap();        

        match stream.write(&*send_buffer.clone().into_bytes()) {
            Ok(send_size) => {
                match stream.read(&mut receive_buffer) {
                    Ok(received_size) => {
                        if send_size != received_size {
                            println!("** STREAM RESEND ERROR **");
                            return
                        }

                        println!("** STREAM PING PONG **");
                        let received_data = &receive_buffer[0..received_size];
                        print(received_data);
                    }
                    Err(_) => {
                        println!("** STREAM STOPPED (WRITE) **");
                        return
                    }
                }
            }
            Err(_) => {
                println!("** STREAM STOPPED (READ) **");
                return
            }
        }
    }
}
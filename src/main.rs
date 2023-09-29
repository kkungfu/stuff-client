use rdev::listen;
use std::net::{TcpListener, TcpStream};
use std::thread::sleep;
use std::time::Duration;
use client::{build_tray, callback, tcp_client, tcp_listen, init_folders, init_status, LOG_FILE};

fn main() {
    // init_folders();

    // *LOG_FILE.lock().unwrap() = init_status();

    // let _tray_icon = build_tray();

    // std::thread::spawn(move || {
    //     if let Err(error) = listen(callback) {
    //         println!("Error: {:?}", error)
    //     }
    // });

    std::thread::spawn(move || {
        println!("-- CLIENT Listening on 30002  --");
        let listener = TcpListener::bind("127.0.0.1:30002").unwrap();
        for stream in listener.incoming() { tcp_listen(stream.unwrap()); }
    });

    loop {
        match TcpStream::connect("127.0.0.1:30000") {
            Ok(stream) => {
                tcp_client(stream);
                break;
            },
            Err(err) => {
                println!("{}", err);
                sleep(Duration::from_secs(1));
            }
        };
    }
}
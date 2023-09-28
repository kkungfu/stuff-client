use rdev::listen;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;
use client::{build_tray, callback, tcp_client, init_folders, init_status, LOG_FILE};

fn main() {
    init_folders();

    *LOG_FILE.lock().unwrap() = init_status();

    let _tray_icon = build_tray();

    std::thread::spawn(move || {
        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error)
        }
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
use rdev::listen;
use std::net::{TcpListener, TcpStream};
use std::thread::sleep;
use std::time::Duration;
use client::{build_tray, callback, tcp_client, tcp_listen, init_folders, init_status, net_client};
use client::LOG_FILE;

fn main() {
    // init_folders();

    // *LOG_FILE.lock().unwrap() = init_status();

    // let _tray_icon = build_tray();

    // std::thread::spawn(move || {
    //     if let Err(error) = listen(callback) {
    //         println!("Error: {:?}", error)
    //     }
    // });

    net_client("CoolNickName");
}
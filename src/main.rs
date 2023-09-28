use rdev::listen;
use std::net::TcpStream;
use tray_icon::{
    menu::MenuEvent,
    TrayIconEvent,
};
use winit::event_loop::{ControlFlow, EventLoopBuilder};
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

    std::thread::spawn(move || {
        let stream = TcpStream::connect("127.0.0.1:30000").unwrap();
        tcp_client(stream);
    });

    let event_loop = EventLoopBuilder::new().build();
    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if let Ok(event) = tray_channel.try_recv() {
            println!("{event:?}");
            match event.click_type {
                tray_icon::ClickType::Left => (),
                _ => ()
            }
        }
        if let Ok(event) = menu_channel.try_recv() {
            println!("menu event: {:?}", event);
        }
    });
}
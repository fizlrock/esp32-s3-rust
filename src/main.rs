mod freq_meter;
mod init;
mod led;
mod wifi;
mod wsserver;
use init::init_device;
use log::info;
use tungstenite::Error;
use wsserver::handle_client;

use std::net::TcpListener;

fn main() {
    let mut devices = init_device();

    devices.led.set(0, 50, 0);

    info!("Programm started");

    let ip_addr = devices
        .wifi
        .sta_netif()
        .get_ip_info()
        .unwrap()
        .ip
        .to_string();

    info!("listening on address {ip_addr}:9001");

    let server = TcpListener::bind(ip_addr + ":9001").unwrap();

    for stream in server.incoming() {
        match stream {
            Ok(stream) => {
                info!(
                    "Client connected: {}",
                    stream.peer_addr().unwrap().ip().to_string()
                );
                devices.led.set(112, 64, 25);

                if let Err(err) = handle_client(stream, &mut devices) {
                    match err {
                        Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                        e => info!("test: {}", e),
                    }
                }
                info!("Client disconected");
                devices.led.set(0, 100, 25);
            }
            Err(e) => info!("Error accepting stream: {}", e),
        }
    }
}

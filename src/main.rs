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
    let mut devs = init_device();

    devs.led.set(0, 50, 0);

    info!("Programm started");

    let ip_addr = devs.wifi.sta_netif().get_ip_info().unwrap().ip.to_string();

    info!("listening on address {ip_addr}:9001");

    let server = TcpListener::bind(ip_addr + ":9001").unwrap();

    for stream in server.incoming() {
        info!("Client connected ",);

        devs.led.set(112, 64, 25);
        match stream {
            Ok(stream) => {
                if let Err(err) = handle_client(stream, &mut devs) {
                    match err {
                        Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                        e => info!("test: {}", e),
                    }
                }
            }
            Err(e) => info!("Error accepting stream: {}", e),
        }
        devs.led.set(0, 200, 25);
        info!("Client disconected");
    }
}

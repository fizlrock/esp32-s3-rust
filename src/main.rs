use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{delay::FreeRtos, prelude::Peripherals},
    sys::nvs_flash_init,
};

mod led;
mod wifi;
use led::WS2812RMT;
use log::info;
use wifi::init_wifi_with_defaults;

use std::{borrow::Cow, net::TcpListener};
use tungstenite::{accept, protocol::frame::coding::CloseCode};

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("System started!");
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take().unwrap();

    // Onboard RGB LED pin
    // Rust ESP Board gpio2,  ESP32-C3-DevKitC-02 gpio8
    let led = peripherals.pins.gpio48;
    let channel = peripherals.rmt.channel0;
    let mut ws2812 = WS2812RMT::new(led, channel).unwrap();

    ws2812.set_pixel(rgb::RGB8::new(0, 0, 50)).unwrap();

    unsafe { nvs_flash_init() };

    let _wifi = wifi::wifi("SpecialForYou", "GtuuhHI7Gg", peripherals.modem, sysloop).unwrap();

    ws2812.set_pixel(rgb::RGB8::new(0, 50, 0)).unwrap();
    info!("Programm started");

    let server = TcpListener::bind("192.168.0.27:9001").unwrap();
    for stream in server.incoming() {
        let the_client = stream.unwrap();

        ws2812.set_pixel(rgb::RGB8::new(50, 50, 0)).unwrap();
        println!("Client: {}", the_client.peer_addr().unwrap());
        let mut websocket = accept(the_client).unwrap();

        websocket
            .write(tungstenite::Message::Text("Hello client".to_string()))
            .unwrap();

        let close_frame = tungstenite::protocol::frame::CloseFrame {
            code: CloseCode::Normal,
            reason: Cow::Borrowed("Goodby"),
        };
        websocket
            .close(Some(close_frame))
            .expect("Failed to close connection");

        let _ = websocket.flush();
        ws2812.set_pixel(rgb::RGB8::new(0, 50, 0)).unwrap();
    }
}

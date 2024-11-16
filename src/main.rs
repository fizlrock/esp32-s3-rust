use esp_idf_svc::{hal::delay::FreeRtos, sys::nvs_flash_init};
use log::info;

mod wifi;
use wifi::init_wifi_with_defaults;

use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;

use esp_idf_sys::esp_pthread_cfg_t;

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    // let mut pthread_cfg = esp_pthread_cfg_t::default();
    // // pthread_cfg.thread_stack_size = 8192; // Увеличенный размер стека
    // pthread_cfg.

    // unsafe {
    //     esp_idf_sys::esp_pthread_set_cfg(&pthread_cfg);
    // }

    unsafe { nvs_flash_init() };
    let _wifi = init_wifi_with_defaults();

    info!("Programm started");

    let server = TcpListener::bind("192.168.0.27:9001").unwrap();
    for stream in server.incoming() {
        let the_client = stream.unwrap();
        println!("Client: {}", the_client.peer_addr().unwrap());
        let mut websocket = accept(the_client).unwrap();

        websocket
            .write(tungstenite::Message::Text("Hello client".to_string()))
            .unwrap();
        let _ = websocket.flush();
    }

    // let mut counter: u32 = 0;
    // loop {
    //     println!("hey {} ", counter);
    //     FreeRtos::delay_ms(500);
    //     counter = counter + 1;
    // }

    // let server = TcpListener::bind("192.168.0.27:9001").unwrap();
    // for stream in server.incoming() {
    //     spawn(move || {
    //         let the_client = stream.unwrap();
    //         println!("Client: {}", the_client.local_addr().unwrap());
    //         let mut websocket = accept(the_client).unwrap();

    //         websocket
    //             .write_message(tungstenite::Message::Text("Hello client".to_string()))
    //             .unwrap();

    //         loop {
    //             if websocket.can_read() {
    //                 let msg = websocket.read_message().unwrap();

    //                 if msg.is_binary() || msg.is_text() {
    //                     println!("Message received: {}", msg.clone().into_text().unwrap());
    //                 }
    //             }
    //         }
    //     });
    // }
}

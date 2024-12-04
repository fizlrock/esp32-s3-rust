use std::net::TcpStream;

use tungstenite::{accept, handshake::HandshakeRole, Error, HandshakeError, Message, Result};

use crate::init::Devices;

fn must_not_block<Role: HandshakeRole>(err: HandshakeError<Role>) -> Error {
    match err {
        HandshakeError::Interrupted(_) => panic!("Bug: blocking socket would block"),
        HandshakeError::Failure(f) => f,
    }
}

enum Command {
    SetLedColor { channel: char, value: u8 },
    GetVoltage { channel: u8 },
    GetConterValue,
    Error,
}

fn parse_command(command: &str) -> Command {
    if command.len() == 0 {
        return Command::Error;
    }

    let cmd = String::from(command);

    match &cmd[0..1] {
        "l" => {
            // lg234
            let channel = &cmd[1..2].chars().next().unwrap();
            let value = &cmd[2..].to_string().parse::<u8>().unwrap();
            return Command::SetLedColor {
                channel: *channel,
                value: *value,
            };
        }
        "v" => {
            // lg234
            let value = &cmd[1..].to_string().parse::<u8>().unwrap();
            return Command::GetVoltage { channel: *value };
        }
        "c" => Command::GetConterValue,
        _ => {
            println!("something else: {command}!");
            Command::Error
        }
    }
}

pub fn handle_client(stream: TcpStream, devs: &mut Devices) -> Result<()> {
    let mut socket = accept(stream).map_err(must_not_block)?;
    loop {
        match socket.read()? {
            msg @ Message::Text(_) => {
                let text = msg.to_text().unwrap().trim();
                let parsed_command = parse_command(text);

                match parsed_command {
                    Command::Error => {
                        socket.send(Message::Text("Ошибка распознование команды".to_owned()))?;
                    }

                    Command::SetLedColor { channel, value } => match channel {
                        'r' => devs.led.set_red(value),
                        'g' => devs.led.set_green(value),
                        'b' => devs.led.set_blue(value),
                        _ => {}
                    },
                    Command::GetVoltage { channel } => {
                        let lsb = devs.adc_pin.read().unwrap();
                        const LSB_PRICE: f32 = 3900.0 / 4096.0;
                        let voltage = (lsb as f32) * LSB_PRICE;
                        socket.send(Message::Text(format!("ADC value: {:4.0} mv", voltage)))?;
                    }
                    Command::GetConterValue => {
                        let counter_value = devs.pcnt.get_counter_value().unwrap();
                        let text = format!("counter value: {counter_value}");
                        let msg = Message::text(text);

                        socket.send(msg)?;
                    } // _ => execute_command(parsed_command, devs),
                }
            }
            Message::Ping(_) | Message::Pong(_) | Message::Frame(_) | Message::Close(_) => {}
            Message::Binary(_) => {
                socket
                    .send(Message::Text("не шли мне бинарные данные".to_owned()))
                    .unwrap();
            }
        }
    }
}

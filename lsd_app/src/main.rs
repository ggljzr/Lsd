extern crate serialport;

use std::io::{self, Write};
use std::time::Duration;

use serialport::prelude::*;
#[derive(Debug)]
enum Command {
    INIT {cols: u8, rows: u8}, //command for display initialization
    WRITE(u8), //writes byte on screen, increments cursor
    SET_C{col: u8, row: u8}, //sets screen to position
    CLEAR, //clears display
    HOME, //sets cursor to 0 0
}

fn parse_command(data: &Vec<u8>) -> Result<Command, io::Error> {
    println!("{:?}", data);
    Ok(Command::INIT{cols: 16, rows: 2})
}

fn main() {
    let mut port_name = "COM3".to_string();
    let mut baud_rate = "9600".to_string();

    let mut settings: SerialPortSettings = Default::default();
    settings.timeout = Duration::from_millis(10);
    if let Ok(rate) = baud_rate.parse::<u32>() {
        settings.baud_rate = rate.into();
    } else {
        eprintln!("Error: Invalid baud rate '{}' specified", baud_rate);
        ::std::process::exit(1);
    }

    match serialport::open_with_settings(&port_name, &settings) {
        Ok(mut port) => {
            let mut serial_buf: Vec<u8> = vec![0; 16];
            println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
            loop {
                match port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => { 
                        let command = parse_command(&serial_buf); 
                        println!("{:?}", command);
                        () 
                    },
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        },
        Err(e) => {
            eprintln!(
                "Failed to open \"{}\". Error: {}",
                port_name,
                e
            );
            ::std::process::exit(1);
        },
    }
}

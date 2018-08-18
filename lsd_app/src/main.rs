extern crate serialport;
extern crate lsd_app;

use std::io::{self, Write};
use std::time::Duration;

use serialport::prelude::*;

use lsd_app::display::Display;

fn main() {

    let mut d = Display::new(16, 2);
    
    d.set_cursor(0, 1);

    let mut i: u8 = 1;

    let mut w = lsd_app::display_window::DisplayWindow::new();
    
    loop {
        d.set_cursor(0, 1);
        d.write_byte(65 + (i % 26));
        i += 1;

        match w.draw(&d.to_string()) {
            Some(()) => {},
            None => {
                println!("Exiting...");
                break;
            }
        }

        std::thread::sleep_ms(200);
    }

    /*
    let mut port_name = "COM4".to_string();
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
                        let command = lsd_app::parse_command(&serial_buf); 
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

    */
}

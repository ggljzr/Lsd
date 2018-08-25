extern crate lsd_app;
extern crate serialport;
extern crate argparse;

use serialport::prelude::*;
use argparse::{ArgumentParser, Store};

use lsd_app::display::{Display, parse_command, COMMAND_SIZE};
use lsd_app::display_window::DisplayWindow;

use std::time::Duration;

fn main() {
    let mut d = Display::new(16, 2);
    let mut w = DisplayWindow::new();
    let mut glyphs = w.get_glyphs();

    let mut port_name = "COM4".to_string();
    let mut baud_rate = "9600".to_string();

    {
        let mut parser = ArgumentParser::new();
        parser.set_description("LSD display application");
        parser.refer(&mut port_name)
        .add_argument("port", Store, "Serial port to which is Arduino connected (e. g. COM4)");
        parser.refer(&mut baud_rate)
        .add_argument("baud_rate", Store, "Baud rate");

        parser.parse_args_or_exit();
    }

    let mut settings: SerialPortSettings = Default::default();
    settings.timeout = Duration::from_millis(10);
    if let Ok(rate) = baud_rate.parse::<u32>() {
        settings.baud_rate = rate.into();
    } else {
        eprintln!("Error: Invalid baud rate '{}' specified", baud_rate);
        ::std::process::exit(1);
    }

    /*
    based on https://gitlab.com/susurrus/serialport-rs/blob/master/examples/receive_data.rs
    */

    match serialport::open_with_settings(&port_name, &settings) {
        Ok(mut port) => {
            let mut serial_buf: Vec<u8> = vec![0; COMMAND_SIZE];
            println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
            loop {
                match port.read_exact(serial_buf.as_mut_slice()) {
                    Ok(_) => {
                        match parse_command(&serial_buf) {
                            Ok(cmd) => d.exec_command(cmd).unwrap(),
                            //Ok(cmd) => println!("{:?} ({:?})", cmd, chunk),
                            Err(_e) => println!("Invalid cmd ({:?})", &serial_buf),
                        }

                    },
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }

                match w.draw(d.get_buffer(), &mut glyphs) {
                    Some(()) => {},
                    None => {
                        println!("Exiting...");
                        break;
                    }
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

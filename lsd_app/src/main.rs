extern crate lsd_app;
extern crate serialport;

use serialport::prelude::*;

use lsd_app::display::Display;
use lsd_app::display::parse_command;
use std::time::Duration;

fn main() {
    let mut d = Display::new(16, 2);

    let mut i: u8 = 1;

    let mut w = lsd_app::display_window::DisplayWindow::new();
    let mut glyphs = w.get_glyphs();

    /*
    loop {
        d.set_cursor(0, 1).unwrap();
        d.write_byte(65 + i);
        i = (i + 1) % 26;

        match w.draw(d.get_buffer(), &mut glyphs) {
            Some(()) => {}
            None => {
                println!("Exiting...");
                break;
            }
        }
    }*/

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
            let mut serial_buf: Vec<u8> = vec![10; 1000];
            println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
            loop {
                match port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => {
                        if t % 3 != 0 {
                            continue;
                        }

                        for chunk in serial_buf[..t].chunks(3) {
                            match parse_command(chunk) {
                                Ok(cmd) => d.exec_command(cmd).unwrap(),
                                //Ok(cmd) => println!("{:?} ({:?})", cmd, chunk),
                                Err(e) => println!("Invalid cmd ({:?})", chunk),
                            }
                            //println!("{:?}", &cmd);
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

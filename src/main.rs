extern crate serialport;

use serialport::prelude::*;
use std::env;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    let mut send_string = match cli_args.get(1) {
        Some(strng) => strng.clone(),
        None => panic!("No send string supplied as argument"),
    };
    //send_string.push_str("\n");
    if !send_string.is_ascii() {
        panic!("Only ascii in argument");
    }
    //let settings = SerialPortSettings::new();
    let mut port = match serialport::open("/dev/ttyACM0") {
        Ok(p) => p,
        Err(_e) => match serialport::open("/dev/ttyACM1") {
            Ok(p) => p,
            Err(e) => panic!("Could not open /dev/ttyACM0 or /dev/ttyACM1\n{}", e),
        },
    };

    while port.write(send_string.as_bytes()).is_err() {
        sleep(Duration::from_millis(100));
    }

    let mut recieve_string = String::from("");
    port.read_to_string(&mut recieve_string);
    while !(recieve_string.contains("OK")) {
        println!(
            "NOT OK: sent {} and recieved {}",
            send_string, recieve_string
        );
        sleep(Duration::from_millis(1000));
        while port.write(send_string.as_bytes()).is_err() {
            sleep(Duration::from_millis(100));
        }
        port.read_to_string(&mut recieve_string);
    }
    if recieve_string == "OK" {
        println!("OK: sent {} and recieved {}", send_string, recieve_string);
    };
    println!("sent {} and recieved {}", send_string, recieve_string);
}

use std::process;

use rppal::gpio::{Gpio, OutputPin};

pub struct BinaryWrapper {
    pub pin1: OutputPin,
    pub pin2: OutputPin,
    pub pin3: OutputPin,
    pub pin4: OutputPin,
}

impl BinaryWrapper {
    pub fn new(port1: u8, port2: u8, port3: u8, port4: u8) -> BinaryWrapper {
        let gpio = match Gpio::new() {
            Ok(v) => v,
            _ => {
                println!("Error creating GPIO struct.");
                process::exit(1);
            }
        };

        let pin1 = match gpio.get(port1) {
            Ok(v) => v,
            _ => {
                println!("Error creating the first Pin.");
                process::exit(1);
            }
        };
        let pin1 = pin1.into_output();

        let pin2 = match gpio.get(port2) {
            Ok(v) => v,
            _ => {
                println!("Error creating the second Pin.");
                process::exit(1);
            }
        };
        let pin2 = pin2.into_output();

        let pin3 = match gpio.get(port3) {
            Ok(v) => v,
            _ => {
                println!("Error creating the second Pin.");
                process::exit(1);
            }
        };
        let pin3 = pin3.into_output();

        let pin4 = match gpio.get(port4) {
            Ok(v) => v,
            _ => {
                println!("Error creating the second Pin.");
                process::exit(1);
            }
        };
        let pin4 = pin4.into_output();

        BinaryWrapper { pin1: pin1, pin2: pin2, pin3: pin3, pin4: pin4 }
    }
}

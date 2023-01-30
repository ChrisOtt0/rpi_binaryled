mod binary_wrapper;

use std::{process, io::{self, Write}};

use binary_wrapper::BinaryWrapper;

fn main() {
    let port1: u8 = 18;
    let port2: u8 = 24;
    let port3: u8 = 12;
    let port4: u8 = 16;
    let mut bw: BinaryWrapper = BinaryWrapper::new(port1, port2, port3, port4);
    let functions: Vec<fn(&mut BinaryWrapper)> = vec![
        zero,
        one,
        two,
        three,
        four,
        five,
        six,
        seven,
        eight,
        nine,
        ten,
        eleven,
        twelve,
        thirteen,
        fourteen,
        fifteen,
        close,
    ];

    loop {
        let mut input = String::new();
        print!("Please insert a number between 0-15 (16 to exit): ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut input).expect("Error reading from STDIN.");
        
        match input.parse::<usize>() {
            Ok(v) => {
                if v > 16 {
                    println!("Unknown request. Please try again.");
                } else {
                    functions[v](&mut bw);
                }
            },
            _ => println!("Unknown request. Please try again."),
        }
    }
}

fn close(bw: &mut BinaryWrapper) {
    zero(bw);
    process::exit(0);
}

fn zero(bw: &mut BinaryWrapper) {
    bw.pin1.set_low();
    bw.pin2.set_low();
    bw.pin3.set_low();
    bw.pin4.set_low();
}

fn one(bw: &mut BinaryWrapper) {
    bw.pin1.set_high();
    bw.pin2.set_low();
    bw.pin3.set_low();
    bw.pin4.set_low();
}

fn two(bw: &mut BinaryWrapper) {
    bw.pin1.set_low();
    bw.pin2.set_high();
    bw.pin3.set_low();
    bw.pin4.set_low();
}

fn three(bw: &mut BinaryWrapper) {
    bw.pin1.set_high();
    bw.pin2.set_high();
    bw.pin3.set_low();
    bw.pin4.set_low();
}

fn four(bw: &mut BinaryWrapper) {
    bw.pin1.set_low();
    bw.pin2.set_low();
    bw.pin3.set_high();
    bw.pin4.set_low();
}

fn five(bw: &mut BinaryWrapper) {
    bw.pin1.set_high();
    bw.pin2.set_low();
    bw.pin3.set_high();
    bw.pin4.set_low();
}

fn six(bw: &mut BinaryWrapper) {
    bw.pin1.set_low();
    bw.pin2.set_high();
    bw.pin3.set_high();
    bw.pin4.set_low();
}

fn seven(bw: &mut BinaryWrapper) {
    bw.pin1.set_high();
    bw.pin2.set_high();
    bw.pin3.set_high();
    bw.pin4.set_low();
}

fn eight(bw: &mut BinaryWrapper) {
    bw.pin1.set_low();
    bw.pin2.set_low();
    bw.pin3.set_low();
    bw.pin4.set_high();
}

fn nine(bw: &mut BinaryWrapper) {
    bw.pin1.set_high();
    bw.pin2.set_low();
    bw.pin3.set_low();
    bw.pin4.set_high();
}

fn ten(bw: &mut BinaryWrapper) {
    bw.pin1.set_low();
    bw.pin2.set_high();
    bw.pin3.set_low();
    bw.pin4.set_high();
}

fn eleven(bw: &mut BinaryWrapper) {
    bw.pin1.set_high();
    bw.pin2.set_high();
    bw.pin3.set_low();
    bw.pin4.set_high();
}

fn twelve(bw: &mut BinaryWrapper) {
    bw.pin1.set_low();
    bw.pin2.set_low();
    bw.pin3.set_high();
    bw.pin4.set_high();
}

fn thirteen(bw: &mut BinaryWrapper) {
    bw.pin1.set_high();
    bw.pin2.set_low();
    bw.pin3.set_high();
    bw.pin4.set_high();
}

fn fourteen(bw: &mut BinaryWrapper) {
    bw.pin1.set_low();
    bw.pin2.set_high();
    bw.pin3.set_high();
    bw.pin4.set_high();
}

fn fifteen(bw: &mut BinaryWrapper) {
    bw.pin1.set_high();
    bw.pin2.set_high();
    bw.pin3.set_high();
    bw.pin4.set_high();
}

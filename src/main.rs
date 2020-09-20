use std::env;
use std::{thread, time};

fn main() {
    //loop on CLI parameters
    for argument in env::args() {
        match argument.parse::<u64>() {
            Err(e) => {
                println!("parameter {} is not a number -> {}", argument, e);
            }
            Ok(v) => {
                println!("waiting {} seconds", v);
                let millis = time::Duration::from_millis(v * 1000);
                thread::sleep(millis);
                return;
            }
        };
    }
    println!("Synthaxe: wait n");
    println!("Sleep for n seconds");
}

use std::{env};
use std::error::Error;

mod asciiart;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: asciiart [file ...]");
        return ();
    } else {
        for arg in &args[1..] {
            asciiart::to_ascii_art(arg);
        }
    }

}

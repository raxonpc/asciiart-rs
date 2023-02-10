use std::{env};

mod asciiart;

use asciiart::{ ProcessingError };

fn main() -> Result<(), ProcessingError>{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: asciiart file scale");
        return Ok(());
    } else {
        let mut scale: f64 = 1.0;
        if let Some(s) = args.get(2) {
            scale = s.parse().unwrap();
        }
        let output = asciiart::to_ascii_art(args.get(1).unwrap(), Some(scale))?;
        println!("{}", &output);
    }
    Ok(())
}

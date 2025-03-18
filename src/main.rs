#![allow(dead_code)]

mod chip_8;
use std::io;

fn main() -> Result<(), io::Error>{

    println!("Please enter the name of the chip-8 rom:");
    let mut filename = String::new();
    io::stdin().read_line(&mut filename)?;
    filename = filename.trim_end().to_string();

    let mut chip_8 = chip_8::Chip8::new(filename.as_str(), 640, 320);
    chip_8.run_chip_8();
    Ok(())
}

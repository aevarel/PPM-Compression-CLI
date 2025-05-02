// File: main.rs
pub mod codec;
pub mod image;
pub mod rgb_cvc;
pub mod cvc_block;
pub mod blocks_bwc;
pub mod utils;
use std::env;

/// The main function of the program, which takes in command line arguments and calls the appropriate function
fn main() {
    let args: Vec<String> = env::args().collect();
    let argnum = args.len();
    assert!(argnum == 2 || argnum == 3);
    let filename = if argnum == 3 {
        Some(args[2].clone())
    } else {
        None
    };
    
    match args[1].as_str() {
        "-c" => codec::compress(filename.as_deref()),
        "-d" => codec::decompress(filename.as_deref()),
        // debug function
        "-r" => codec::round_trip(filename.as_deref()), 
        _ => {
            eprintln!("Usage:\nrpeg -d [filename]\nrpeg -c [filename]\nrpeg -c\n rpeg -d");
        }
    }        
}
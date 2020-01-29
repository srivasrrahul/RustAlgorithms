use std::io::{BufRead, BufReader};
use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind;


fn read_pattern(file_full_name : &str,pattern : &str) -> std::io::Result<(String)> {
    let mut file = File::open(file_full_name)?;
    let mut buffer : Vec<u8> = [0;1].to_vec();
    let bytes_pattern = String::from(pattern).into_bytes();
    while true {
        match file.read_exact(&mut buffer) {
            Ok(()) => println!("read data {}",buffer[0] as char),
            _ => {
                break
            }
        };
    }
    
    
    return Ok(String::from(""));
    

}

fn main() {
    read_pattern("./test.txt","hello");
}
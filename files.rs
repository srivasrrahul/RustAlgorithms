use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::Write;
use std::io::{self,BufReader,BufRead};
use std::io::prelude::*;
fn store_data(data : &str,file_full_name : &str) {
    let path = Path::new(file_full_name);
    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(why) => panic!("issue creating file {}",why.description())
    };

    match file.write_all(data.as_bytes()) {
        Ok(_) => println!("Wrote files "),
        Err(why) => panic!("issue creating file {}",why.description())
    };
}

fn read_data(file_full_name : &str) -> std::io::Result<(String)> {
    // let path = Path::new(file_full_name);
    // let mut file : File  = match File::open(&path) {
    //     Ok(file) => file,
    //     Err(why) => panic!("issue opening file {}",why.description())

    // };

    let mut file = File::open(file_full_name)?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("Content is {}",contents);

    //return String::from("hello");
    return Ok((contents));

    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;
    // //Ok(())
    // return contents.clone();



    // for line in reader.lines() {
    //     println!("{}",line)
    // }



}

fn main() {
    store_data("Hello World","/Users/rasrivastava/RUST/rust-learning/src/test.txt");
    match read_data("/Users/rasrivastava/RUST/rust-learning/src/test.txt") {
        Ok(s) => println!("data is {}",s),
        _ => println!("failed")
    };
}
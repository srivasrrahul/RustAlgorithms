use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::LinkedList;

fn tail(file_full_name : &str,n : usize) -> std::io::Result<LinkedList<String>> {
    let mut ret_value : LinkedList<String> = LinkedList::new();
    let file = File::open(file_full_name).unwrap();
    let reader = BufReader::new(file);
    for (index,line) in reader.lines().enumerate() {
        let current_line = line.unwrap(); 
        if ret_value.len() >= n {
            ret_value.pop_front();
        }

        ret_value.push_back(current_line);
        

    }
    return Ok(ret_value);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_full_name = String::from(&args[1]);
    let line_count = String::from(&args[2]).parse::<usize>().unwrap();
    let lst = tail(&file_full_name,line_count).unwrap();
    println!("File name : {} , Line count is {}",file_full_name,line_count);
    for l in lst {
        println!("{}",l);
    }
}
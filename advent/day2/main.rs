use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines, Result};

fn read_lines(file_path: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(file_path)?;
    Ok(BufReader::new(file).lines())
}

fn main() {
    let mut data: Vec<i32> = Vec::new();
    match read_lines("./data.txt") {
        Ok(lines) => {
            for line in lines {
                if let Ok(d) = line {
                    let dt: Vec<&str>= d.split(" ").collect();
                    data = dt.iter().parse::<i32>().unwrap().collect();
                    println!("print data {:?} \n", data[0]);
                }
            }
        }
        Err(e) => println!("Error reading file: {}", e),
    }
    //println!("print data {:?}", data);
}
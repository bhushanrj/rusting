use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines, Result};

fn read_lines(file_path: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(file_path)?;
    Ok(BufReader::new(file).lines())
}

fn main() {
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();
    match read_lines("./data.csv") {
        Ok(lines) => {
            for line in lines {
                if let Ok(ip) = line {
                    let mut d: Vec<&str>= ip.split("   ").collect();
                    let mut val: i32 = d.pop().unwrap().parse().expect("Falied to read");
                    col1.push(val);
                    val  = d.pop().unwrap().parse().expect("failed to read");
                    col2.push(val);
                }
            }
        }
        Err(e) => println!("Error reading file: {}", e),
    }

    // sort
    col1.sort();
    col2.sort();

    // part 1:
    let mut diff: Vec<i32> = Vec::new();
    for n in 0..col1.len() {
        diff.push((col1[n] - col2[n]).abs());
    }
    let mut total: i32 = 0;
    for n in 0..diff.len() {
        total = total + diff[n];
    }
    println!("part1 answer {:?}\n", total);

    // part 2:
    let mut data: Vec<i32> = Vec::new();
    let mut count: i32;
    for n in 0..col1.len() {
        count = 0;
        for m in 0..col2.len() {
            if col1[n] == col2[m] {
                count = count + 1;
            }
        }
        data.push(count * col1[n]);
    }

    //println!("data {:?}, len : {}", data, data.len());
    let mut count: i32 = 0;
    for n in 0..data.len(){
        count = count + data[n];
    }
    println!("part2 answer: {}\n", count);

}
#![allow(dead_code, unused_variables)]

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines, Result};

fn read_lines(file_path: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(file_path)?;
    Ok(BufReader::new(file).lines())
}

fn analyze_report(data: &Vec<i32>) -> bool {

    if data.windows(2).all(|x|x[0] < x[1]) {
       if data.windows(2).all(|x|(x[1] - x[0]).abs() <= 3) {
            println!("t1");
            true
        } else {
            println!("f1");
            false
        }
    } else if data.windows(2).all(|x|x[0] > x[1]) {
        if data.windows(2).all(|x|(x[0] - x[1]).abs() <= 3) {
            println!("t2");
            true
        } else {
            println!("f2");
            false
        }
    } else {
        println!("f3");
        false
    }
}

fn dampen_report(data: &Vec<i32>) -> bool {
    for i in 0..data.len() {
        let mut play_data = data.to_vec();
        if analyze_report(play_data.retain(|&x| x != i)) {
            return true;
        }
    }
    false
}

fn main() {
    let mut report: u32 = 0;
    match read_lines("./data.txt") {
        Ok(lines) => {
            for line in lines {
                if let Ok(d) = line {
                    let dt: Vec<&str> = d.split(" ").collect();
                    println!("print dt {:?} \n", dt);
                    let data_res  = dt.iter().map(|x| x.parse::<i32>()).collect();
                    match data_res {
                        Ok(data) => {
                            let res = analyze_report(&data);
                            println!("print res {} \n", res);
                            if res  {
                                report = report + 1;
                            } else {
                                if dampen_report(&data) {
                                    report = report + 1;
                                }
                            }
                        }
                        Err(e) => eprintln!("Failed to convert: {}\n", e)
                    }
                }
            }
        }
        Err(e) => eprintln!("Error reading file: {}\n", e),
    }
    println!("Report {}\n", report);
}
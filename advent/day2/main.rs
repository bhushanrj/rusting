use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines, Result};

fn read_lines(file_path: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(file_path)?;
    Ok(BufReader::new(file).lines())
}


fn analyze_report(data: &Vec<i32>) -> bool {
    if data.windows(2).all(|x|x[0] < x[1]) {
        if data.windows(2).all(|x|x[1] - x[0] <=0 ) {
            false
        } else if data.windows(2).all(|x|x[1] - x[0] > 3) {
            false
        } else {
            true
        }
    } else if data.windows(2).all(|x|x[0] > x[1]) {
        if data.windows(2).all(|x|x[0] - x[1] <=0 ) {
            false
        } else if data.windows(2).all(|x|x[0] - x[1] > 3) {
            false
        } else {
            true
        }
    } else {
        false
    }
}

fn main() {
    let mut report: u32 = 0;
    match read_lines("./data.txt") {
        Ok(lines) => {
            for line in lines {
                if let Ok(d) = line {
                    let dt: Vec<_> = d.split(" ").collect();
                    println!("print dt {:?} \n", dt);
                    let data_res: Result<Vec<i32>, _>  = dt.iter().map(|x| x.parse::<i32>()).collect();
                    match data_res {
                        Ok(data) => {
                            let res = analyze_report(&data);
                            println!("print res {}", res);
                            if res  {
                                report = report + 1;
                            }
                        }
                        Err(e) => eprintln!("Failed to convert: {}", e)
                    }
                }
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    println!("Report {}", report);
}
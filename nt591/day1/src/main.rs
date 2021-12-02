use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    first_part()?;
    second_part()?;
    Ok(())
}

fn first_part() -> io::Result<()> {
    let file = File::open("day1.txt")?;
    let reader = BufReader::new(file);
    let mut last_line: Option<i32> = None;
    let mut count = 0;

    for line in reader.lines() {
        match line?.parse::<i32>() {
            Ok(num) => {
                if let Some(last_num) = last_line {
                    if num > last_num {
                        count += 1;
                    }
                };
                last_line = Some(num);
            }
            Err(_) => {
                panic!("Invalid input")
            }
        }
    }
    println!("Count of increases: {}", count);
    Ok(())
}

fn second_part() -> io::Result<()> {
    let file = File::open("day1.txt")?;
    let reader = BufReader::new(file);
    let line_vec: Vec<i32> = reader
        .lines()
        .map(|line| {
            line.expect("Should be able to read")
                .parse::<i32>()
                .expect("Should be able to parse")
        })
        .collect();
    let mut i = 0;
    let mut k = 2;
    let mut sum = line_vec[0] + line_vec[1] + line_vec[2];
    let mut count = 0;

    while k < line_vec.len() {
        // hold temp reference to the sum so we can evict and increment
        let tmp = sum;
        i = i + 1;
        k = k + 1;
        if k < line_vec.len() {
            sum = sum - line_vec[i -1] + line_vec[k];
            if sum > tmp {
                count = count + 1;
            }
        }
    }

    println!("Counted number of window increases: {}", count);
    Ok(())
}

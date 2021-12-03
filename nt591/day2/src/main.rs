use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    first_part()?;
    second_part()?;
    Ok(())
}

enum Direction {
    Down(i32),
    Up(i32),
    Forward(i32)
}

fn to_direction(input: Vec<&str>) -> Direction {
    let delta = input[1].parse::<i32>().unwrap();
    match input[0] {
        "forward" => Direction::Forward(delta),
        "down" => Direction::Down(delta),
        "up" => Direction::Up(delta),
        _ => panic!()
    }
}

fn first_part() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let data: Vec<String> = reader.lines().collect::<Result<Vec<String>, _>>().unwrap();
    let positions: Vec<Direction> = data
        .iter()
        .map(|pos_string| pos_string.split(" ").collect())
        .map(|str_lst| to_direction(str_lst))
        .collect();
    
    let mut x = 0;
    let mut y = 0;

    for position in positions.iter() {
        match position {
            Direction::Forward(i) => {
                x += i
            },
            Direction::Down(i) => {y += i},
            Direction::Up(i) => {y -= i}
        }
    }

    println!("X is {}, y = {}, total is {}", x, y, x * y);
    Ok(())
}

fn second_part() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let data: Vec<String> = reader.lines().collect::<Result<Vec<String>, _>>().unwrap();
    let positions: Vec<Direction> = data
        .iter()
        .map(|pos_string| pos_string.split(" ").collect())
        .map(|str_lst| to_direction(str_lst))
        .collect();
    
    let mut aim = 0;
    let mut depth = 0;
    let mut x = 0;

    for position in positions.iter() {
        match position {
            Direction::Forward(i) => {
                x += i;
                depth += i * aim;
            },
            Direction::Down(i) => {aim += i},
            Direction::Up(i) => {aim -= i}
        }
    }

    println!("X is {}, y = {}, total is {}", x, depth, x * depth);
    Ok(())
}
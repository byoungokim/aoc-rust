use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use regex::Regex;

fn main() -> io::Result<()> {
    let f = File::open("day5.txt")?;
    let mut buf = BufReader::new(f);

    let mut verticals : HashMap<i32, Vec<(i32,i32)>> = HashMap::new();
    let mut horizontals : HashMap<i32, Vec<(i32,i32)>> = HashMap::new();
    let mut diagonals : Vec<(i32, i32, i32, i32)> = Vec::new();
    let regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    for line in buf.lines(){
        match line {
            Ok(line) => for capture in regex.captures_iter( & line) {
                let point1 = (capture.get(1).unwrap().as_str().parse::<i32>().unwrap(), capture.get(2).unwrap().as_str().parse::<i32>().unwrap());
                let point2 = (capture.get(3).unwrap().as_str().parse::<i32>().unwrap(), capture.get(4).unwrap().as_str().parse::<i32>().unwrap());
                if point1.0 == point2.0 {
                    let col = verticals.entry(point1.0).or_insert(Vec::new());
                    col.push((point1.1, point2.1));
                } else if point1.1 == point2.1 {
                    let row = horizontals.entry(point1.1).or_insert(Vec::new());
                    row.push((point1.0, point2.0));
                } else if (point1.0-point2.0).abs() == (point1.1-point2.1).abs() {
                    println!("{}, {}, {}, {}", point1.0, point1.1, point2.0, point2.1);
                    diagonals.push((point1.0, point1.1, point2.0, point2.1));
                }
            },
            _ => panic!()
        }
    }
    println!("verticals: {}", verticals.len());
    println!("horizontals: {}", horizontals.len());
    println!("diagonals: {}", diagonals.len());

    let mut sol_part1 = 0_i32;
    let mut sol_part2 = 0_i32;
    let mut shared_grid: HashMap<(i32, i32), i32> = HashMap::new();
    for vertical in verticals {
        for (mut start, mut end) in vertical.1 {
            if start > end {
                let mut temp = end;
                end = start;
                start = temp;
            }
            for i in start..=end {
                let intersection = shared_grid.entry((vertical.0, i)).or_insert(0);
                *intersection += 1;
                if *intersection == 2 {
                    sol_part1 += 1;
                    sol_part2 += 1;
                }
            }
        }
    }
    for horizontal in horizontals {
        for (mut start, mut end) in horizontal.1 {
            if start > end {
                let mut temp = end;
                end = start;
                start = temp;
            }
            for i in start..=end {
                let intersection = shared_grid.entry((i, horizontal.0)).or_insert(0);
                *intersection += 1;
                if *intersection == 2 {
                    sol_part1 += 1;
                    sol_part2 += 1;
                }
            }
        }
    }
    for diagonal in diagonals{
        let mut current = (diagonal.0, diagonal.1);
        loop {
            let intersection = shared_grid.entry((current.0, current.1)).or_insert(0);
            *intersection += 1;
            if *intersection == 2 {
                sol_part2 += 1;
            }
            if current.0 == diagonal.2 {
                break;
            }
            if current.0 < diagonal.2 {
                current.0 += 1;
            } else {
                current.0 -= 1;
            }
            if current.1 < diagonal.3 {
                current.1 += 1;
            } else {
                current.1 -= 1;
            }
        }
    }
    for i in 0..=9{
        for j in 0..=9{
            let data = shared_grid.entry((j,i)).or_insert(0);
            print!("{} ", data);
        }
        println!();
    }
    println!("sol1: {}, sol2: {}", sol_part1, sol_part2);
    Ok(())
}
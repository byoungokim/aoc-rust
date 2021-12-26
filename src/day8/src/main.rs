use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn encode(pattern_str: &str) -> i32 {
    let mut pattern= 0_i32;
    for alphabet in pattern_str.chars() {
        match alphabet {
            'a' => pattern += 1,
            'b' => pattern += 1 << 1,
            'c' => pattern += 1 << 2,
            'd' => pattern += 1 << 3,
            'e' => pattern += 1 << 4,
            'f' => pattern += 1 << 5,
            'g' => pattern += 1 << 6,
            _ => {}
        }
    }
    return pattern;
}

fn main() -> io::Result<()> {
    let f = File::open("day8_test.txt")?;
    let buf = BufReader::new(f);

    let mut sol1 = 0_i64;
    let mut patterns = [0;10];
    let mut sol2: i64 = 0;
    for line in buf.lines() {
        let line_str = line.unwrap_or("".to_string());
        println!("{}", line_str);
        let test: Vec<&str> = line_str.split("|").collect();
        println!("{}", test.len());
        let ten_patterns: Vec<&str> = test[0].trim().split(" ").collect();
        for i in 0..ten_patterns.len() {
            match ten_patterns[i].len() {
                2 => {patterns[1]=encode(ten_patterns[i]);},
                3 => {patterns[7]=encode(ten_patterns[i]);},
                4 => {patterns[4]=encode(ten_patterns[i]);},
                7 => {patterns[8]=encode(ten_patterns[i]);},
                _ => {println!("skip: {}", ten_patterns[i])}
            }
        }
        for i in 0..ten_patterns.len() {
            if ten_patterns[i].len() == 6 {
                let pattern = encode(ten_patterns[i]);
                if pattern & patterns[1] == patterns[1] {
                    if pattern & patterns[4] == patterns[4] {
                        patterns[9] = pattern;
                    } else {
                        patterns[0] = pattern;
                    }
                } else {
                    patterns[6] = pattern;
                }
            }
        }
        for i in 0..ten_patterns.len() {
            if ten_patterns[i].len() == 5 {
                let pattern = encode(ten_patterns[i]);
                if pattern & patterns[1] == patterns[1] {
                    patterns[3] = pattern;
                } else {
                    if  pattern & patterns[9] == pattern {
                        patterns[5] = pattern;
                    } else {
                        patterns[2] = pattern;
                    }
                }
            }
        }
        let target_digits: Vec<&str> = test[1].trim().split(" ").collect();
        let mut number = 0_i64;
        for digit in target_digits {
            let digit_pattern = encode(digit);
            for i in 0..10 {
                if digit_pattern == patterns[i] {
                    number *= 10;
                    number += i as i64;
                    match i {
                        1 => sol1+=1,
                        4 => sol1+=1,
                        7 => sol1+=1,
                        8 => sol1+=1,
                        _ => {println!("{}", i)}
                    }
                }
            }
        }
        sol2+=number;
    }
    println!("sol1: {}, sol2: {}", sol1, sol2);
    Ok(())
}
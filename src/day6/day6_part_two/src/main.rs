use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("day6_part_two.txt")?;
    let mut buf = BufReader::new(f);

    let mut input = String::new();
    buf.read_line(&mut input)?;
    let sequence: Vec<i32> = input.trim().split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    let mut map = [0; 9];
    for num in sequence { map[num as usize]+=1; }

    let mut sol_part1 = 0_i64;
    for i in 0..256 {
        let temp = map[0];
        for num in 0..8 {
            map[num] = map[num+1];
        }
        map[8] = temp;
        map[6] += temp;

        if i == 79 {
            sol_part1 = map.iter().sum();
        }
    }
    let sum: i64 = map.iter().sum();
    println!("sol1: {}, sol2: {}", sol_part1, sum);
    Ok(())
}
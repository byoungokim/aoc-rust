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
    for i in 0..map.len() {
        println!("n[{}]: {}", i, map[i]);
    }
    for i in 0..256 {
        let temp = map[0];
        for num in 0..8 {
            map[num] = map[num+1];
        }
        map[8] = temp;
        map[6] += temp;
        println!("i: {}, {}", i, map[8]);
    }
    let mut sum = 0_i64;
    for num in 0..9 {
        sum += map[num];
    }
    println!("len: {}", sum);
    Ok(())
}
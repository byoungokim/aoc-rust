use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("day6_part_one.txt")?;
    let mut buf = BufReader::new(f);

    let mut input = String::new();
    buf.read_line(&mut input)?;
    let mut sequence: Vec<i32> = input.trim().split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    for i in 0..80 {
        for j in 0..sequence.len() {
            if sequence[j] == 0 {
                sequence[j] = 6;
                sequence.push(8);
            } else {
                sequence[j] -= 1;
            }
        }
        println!("i: {}, len: {}", i, sequence.len());
    }
    println!("len: {}", sequence.len());
    Ok(())
}
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let f = File::open("day7.txt")?;
    let mut buf = BufReader::new(f);

    let mut input = String::new();
    buf.read_line(&mut input)?;
    let sequence: Vec<i32> = input.trim().split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    let mut map: HashMap<i32, i32> = HashMap::new();
    for num in sequence {
        let counter = map.entry(num).or_insert(0);
        *counter+=1;
    }

    let mut sol_part1 = i32::MAX;
    let mut sol_part2 = i32::MAX;
    for target_key in *map.keys().min().unwrap()..*map.keys().max().unwrap()+1 {
        let fuel = map.keys().map(|x| (target_key-x).abs()*map[&x]).sum();
        let fuel2 = map.keys().map(|x| (target_key-x).abs()*((target_key-x).abs()+1)*map[&x]/2).sum();
        if fuel < sol_part1 {
            sol_part1 = fuel;
        }
        if fuel2 < sol_part2 {
            sol_part2 = fuel2;
        }
    }

    println!("sol1: {}, sol2: {}", sol_part1, sol_part2);
    Ok(())
}
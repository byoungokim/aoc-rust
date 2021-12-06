use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn analyze_bingo(sequence: &Vec<i32>, bingo: &Vec<i32>) -> (i32, i32) {
    let mut sum = 0;
    for i in 0..bingo.len() {
        sum += bingo[i];
    }
    let mut rows = [0_u8; 5];
    let mut cols = [0_u8; 5];
    let mut n_win = 0;
    'outer: for i in 0..sequence.len() {
        for j in 0..bingo.len() {
            if bingo[j] == sequence[i] {
                print!("{}, ", bingo[j]);
                sum -= bingo[j];
                rows[j / 5] += 1;
                if rows[j / 5] == 5 {
                    n_win = i as i32;
                    break 'outer;
                }
                cols[j % 5] += 1;
                if cols[j % 5] == 5 {
                    n_win = i as i32;
                    break 'outer;
                }
            }
        }
    }
    return (n_win, sum);
}

fn main() -> io::Result<()> {
    let f = File::open("day4_part1.txt")?;
    let mut buf = BufReader::new(f);

    let mut input = String::new();
    buf.read_line(&mut input)?;
    let sequence: Vec<i32> = input.trim().split(',').map(|x| x.parse::<i32>().unwrap()).collect();

    println!("sequence.len {}", sequence.len());
    let mut min_seq = sequence.len() as i32;
    let mut score = 0;
    let mut bingo: Vec<i32> = Vec::new();

    for line in buf.lines() {
        if let Ok(line_str) = line {
            line_str.split(' ').for_each(|x| if x.len() > 0 { bingo.push(x.parse::<i32>().unwrap()) });
            println!("len: {}, line_str: {}", bingo.len(), line_str);
            if bingo.len() == 25 {
                let (n_seq, sum) = analyze_bingo(&sequence, &bingo);
                if n_seq < min_seq {
                    min_seq = n_seq;
                    score = sum * sequence[(min_seq) as usize];
                }
                bingo.clear();
            }
            println!("min_seq: {}, score: {}", min_seq, score);
        }
    }

    Ok(())

    // println!("n_passwords: {}", sequence.len());
}
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn main() {
    let path = Path::new("day1_part2.txt");
    let display = path.display();
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let input_lines = io::BufReader::new(file).lines();

    let mut values: Vec<i32> = Vec::new();
    let mut visit = vec![false; 2021];
    for line in input_lines {
        match line {
            Ok(line) => {
                let value: i32 = line.parse().unwrap();
                values.push(value);
                visit[value as usize] = true;
            },
            Err(_) => {
                panic!("");
            }
        }
    }

    'outer: for i in 1..values.len()-1 {
        for j in i+1..values.len() {
            if values[i]+values[j] <= 2020 && visit[(2020-values[i]-values[j]) as usize] == true {
                println!("{}", values[i]+values[j]);
                println!("{}", values[i]*values[j]*(2020-values[i]-values[j]));
                break 'outer;
            }
        }
    }
}

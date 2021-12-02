use std::fs;

fn main() {
    let contents = fs::read_to_string("day1_part1.txt").unwrap();
    let lines = contents.lines();

    let mut n_increments = 0_i32;
    let mut previous = -1;
    for line in lines {
        let current = line.parse::<i32>().unwrap();
        if previous != -1 && previous < current {
            n_increments+=1;
        }
        previous = current;
    }
    println!("n_increments: {}", n_increments);
}

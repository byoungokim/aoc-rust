use std::fs;

fn main() {
    let contents = fs::read_to_string("day1_part2.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut n_increments = 0_i32;
    let mut previous = -1;
    let mut current = 0;
    for i in 0..lines.len() {
        current += lines[i].parse::<i32>().unwrap();
        if i>1 {
            if previous != -1 && previous < current {
                n_increments+=1;
            }
            println!("{}: {} < {} => {}", i, previous, current, n_increments);
            previous = current;
            current-= lines[i-2].parse::<i32>().unwrap();
        }
    }
    println!("n_increments: {}", n_increments);
}

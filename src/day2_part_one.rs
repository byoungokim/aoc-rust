use std::fs;

fn main(){
    let contents = fs::read_to_string("day2_part1.txt").unwrap();
    let lines = contents.lines();
    let mut n_passwords = 0_i32;
    for line in lines {
        let inputs: Vec<&str> = line.split(' ').collect();
        let min_max = inputs[0].split_once('-').unwrap();
        let min: i32 = min_max.0.parse::<i32>().unwrap();
        let max = min_max.1.parse::<i32>().unwrap();
        let mut count = 0_i32;
        let _target = inputs[1].chars().next();
        for c in inputs[2].chars() {
            if _target == Some(c) {
                count+=1;
            }
            if count > max {
                break;
            }
        }
        if count >= min && count <= max {
            n_passwords+=1;
        }
        println!("min: {}, max: {}, count: {}, n_passwords: {}", min, max, count, n_passwords);
    }
    println!("n_passwords: {}", n_passwords);
}
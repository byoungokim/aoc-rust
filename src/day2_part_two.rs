use std::fs;

fn main(){
    let contents = fs::read_to_string("day2_part2.txt").unwrap();
    let lines = contents.lines();
    let mut n_passwords = 0_i32;
    for line in lines {
        let inputs: Vec<&str> = line.split(' ').collect();
        let min_max = inputs[0].split_once('-').unwrap();
        let min: i32 = min_max.0.parse::<i32>().unwrap() - 1;
        let max = min_max.1.parse::<i32>().unwrap() - 1;
        let _target = inputs[1].chars().next();
        let string: Vec<char> = inputs[2].chars().collect();

        if (_target == Some(string[min as usize])) ^ (_target == Some(string[max as usize])) {
            n_passwords+=1;
        }

        println!("min: {}, max: {}, n_passwords: {}", min, max, n_passwords);
    }
    println!("n_passwords: {}", n_passwords);
}
use std::fs;

fn main(){
    let contents = fs::read_to_string("day2_part1.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut horizontal = 0_i32;
    let mut depth = 0_i32;
    for i in 0..lines.len() {
        let inputs: Vec<&str> = lines[i].split(' ').collect();
        let instruction = inputs[0];
        match instruction {
            "forward" => horizontal+=inputs[1].parse::<i32>().unwrap(),
            "down" => depth+=inputs[1].parse::<i32>().unwrap(),
            "up" => depth-=inputs[1].parse::<i32>().unwrap(),
            _ => println!("no pattern"),
        }
        println!("horizontal: {}, depth: {}, result: {}", horizontal, depth, horizontal*depth);
    }
    println!("final_result: {}", horizontal*depth);
}
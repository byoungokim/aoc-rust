use std::fs;

fn main(){
    let contents = fs::read_to_string("day2_part2.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut horizontal = 0_i32;
    let mut aim = 0_i32;
    let mut depth = 0_i32;
    for i in 0..lines.len() {
        let inputs: Vec<&str> = lines[i].split(' ').collect();
        let instruction = inputs[0];
        match instruction {
            "forward" => {
                let x = inputs[1].parse::<i32>().unwrap();
                horizontal+=x;
                depth+= x*aim;
            },
            "down" => aim+=inputs[1].parse::<i32>().unwrap(),
            "up" => aim-=inputs[1].parse::<i32>().unwrap(),
            _ => println!("no pattern"),
        }
        println!("horizontal: {}, aim: {}, depth: {}, result: {}", horizontal, aim, depth, horizontal*depth);
    }
    println!("final: {}", horizontal*depth);
}
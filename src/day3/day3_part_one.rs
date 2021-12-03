use std::fs;

fn main(){
    let contents = fs::read_to_string("day3_part1.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut ones : Vec<i32> = Vec::new();
    let total_inputs = lines.len() as i32;
    for i in 0..lines.len() {
        if i == 0 {
            ones.resize(lines[i].len(), 0);
        }
        for j in 0..lines[i].len() {
            // println!("{}", lines[i].get(j..j+1).unwrap());
            ones[j]+= lines[i].get(j..j+1).unwrap().parse::<i32>().unwrap();
        }
    }

    let mut gamma = 0_i32;
    let mut epsilon = 0_i32;
    let mut test = 0_i32;
    for i in 0..ones.len() {
        if ones[i] > total_inputs - ones[i] {
            gamma ^= 1_i32;
        } else {
            epsilon ^=1_i32;
        }
        test ^= 1_i32;
        if i<ones.len()-1{
            gamma <<=1;
            epsilon <<=1;
            test <<=1;
        }
    }
    println!("len: {}, test = {}", ones.len(), test);
    println!("final_result: {} * {} = {}", gamma, epsilon, gamma*epsilon);
}
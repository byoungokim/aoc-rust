use std::fs;

fn convert_to_number(oxygen: &str, co2: &str) -> (i32, i32){
  let mut oxygen_num = 0;
  let mut co2_num = 0;
  for i in 0..oxygen.len() {
    if oxygen.get(i..i+1).unwrap() == "1" {
      oxygen_num ^= 1;
    }
    if co2.get(i..i+1).unwrap() == "1" {
      co2_num ^= 1;
    }
    if i < oxygen.len()-1{
      oxygen_num <<= 1;
      co2_num <<= 1;
    }
  }
  return (oxygen_num, co2_num);
}

fn find_oxygen_generator_rating(lines: Vec<&str>)-> &str{
  if lines.len() > 1 {
    let mut candidates:Vec<&str> = lines;
    let mut next_candidates:Vec<&str> = Vec::new();
    for j in 0..candidates[0].len() {
      if candidates.len() == 1 {break;}
      let n_oxygen = candidates.len() as i32;
      let mut n_ones_from_o = 0_i32;
      for i in 0..candidates.len() {
        if candidates[i].get(j..j+1).unwrap() == "1" {
          n_ones_from_o+=1;
        }
      }
      // determine oxygen and co2 bit
      let mut oxygen_bit = "0";
      if n_ones_from_o >= n_oxygen - n_ones_from_o {
        oxygen_bit = "1";
      }

      // check bit
      for i in 0..candidates.len() {
        if candidates[i].get(j..j+1).unwrap() == oxygen_bit {
          next_candidates.push(candidates[i]);
        }
      }
      println!("{}, oxygen: {}", next_candidates.len(), next_candidates[0]);
      candidates = Vec::new();
      candidates.append(&mut next_candidates);
    }
    return candidates[0];
  }
  return "";
}

fn find_co2_scrubber_rating(lines: Vec<&str>)-> &str{
  let mut co2: &str = "0";
  if lines.len() > 0 {
    let mut n_co2 = lines.len() as i32;
    let mut is_co2 :Vec<bool> = Vec::new();
    is_co2.resize(n_co2 as usize, true);
    for j in 0..lines[0].len() {
      if n_co2 == 1 {
        break;
      }
      let mut n_ones_from_co = 0_i32;
      for i in 0..lines.len() {
        if lines[i].get(j..j+1).unwrap() == "1" {
          n_ones_from_co+=is_co2[i] as i32;
        }
      }
      // determine oxygen and co2 bit
      println!("n_co2: {}, n_ones: {}", n_co2, n_ones_from_co);
      let mut co2_bit = "1";
      if n_ones_from_co >= n_co2 - n_ones_from_co {
        co2_bit = "0";
      }

      // check bit
      for i in 0..lines.len() {
        if is_co2[i] && (lines[i].get(j..j+1).unwrap() != co2_bit) {
          n_co2 -= 1;
          is_co2[i] = false;
        }
        if is_co2[i] {
          co2 = lines[i];
        }
      }
      println!("co2: {}", co2);
    }
  }
  return co2;
}

fn main(){
  let contents = fs::read_to_string("day3_part2.txt").unwrap();
  let lines: Vec<&str> = contents.lines().collect();

  let oxygen = find_oxygen_generator_rating(lines.clone());
  let co2 = find_co2_scrubber_rating(lines);

  let (oxygen_num, co2_num) = convert_to_number(oxygen, co2);
  println!("final_result: {} * {} = {}", oxygen_num, co2_num, oxygen_num*co2_num);
}
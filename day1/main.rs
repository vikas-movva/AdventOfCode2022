use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
  // get data from text file
  let file = File::open("./day1_input.txt").unwrap();
  let mut elves = Vec::new();
  let mut nums = Vec::new();
  let mut temp = Vec::new();
  
  for line in BufReader::new(file).lines() {
    let line = line.unwrap();

    if line == "" {
  
      elves.push(temp);
      temp = Vec::new();
    
    } else {
      temp.push(line.parse::<i32>().unwrap());
    }
    
  }

  // create a vec that has the length of the number of elves
  nums = vec![0; elves.len()];

  // loop through each elf
  for (i, elf) in elves.iter().enumerate() {
      // assign the sum of the elf's numbers to the corresponding index in nums
      nums[i] = elf.iter().sum();
  }

  // sort nums in descending order  
  nums.sort_unstable_by(|a, b| b.cmp(a));

  // q2 ------------------------------------------------
  // get the sum of the top 3 numbers
  let cals = nums[0] + nums[1] + nums[2];

  // print the sum
  println!("{}", cals);
}

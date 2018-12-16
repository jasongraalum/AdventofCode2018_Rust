use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::borrow::Cow;

pub fn day2_part2(filename: &str) -> io::Result<Cow<str>> {
 let f = File::open(filename)?;
 let f = BufReader::new(f);
 let mut result = String::new();
 // Get first line
 let mut lines : Vec<String> = Vec::new();
 for l in f.lines() 
 {
   let l1 = l?;
   for l2 in &lines
   {
    result = "".to_string();
    let mut diff = 0;
    let mut c2 = l2.chars();
    for c1 in l1.chars() {
      if c1 != c2.next().unwrap() { diff += 1;} else { result.push(c1);}
    }
    if diff == 1 { 
      return Ok(Cow::Owned(result));
    }
   }
   lines.push(l1);
 }
 return Ok(Cow::Owned(result));
}


pub fn day2_part1(filename : &str) -> io::Result<i32> {

  let f = File::open(filename)?;
  let f = BufReader::new(f);

  let mut total_two = 0;
  let mut total_three = 0;

  // For each line of chars
  for line in f.lines() {
      let a_index = 'a' as u32;

      let mut three = 0;
      let mut two = 0;

      let l = &line.unwrap();
      let mut array: [u32; 26] = [0; 26];
      for c in l.chars() {
        let i = (c as u32) - a_index;
        array[i as usize] += 1 ;
        if array[i as usize] == 2 {
          two += 1;
        }
        else if array[i as usize] == 3 {
          two -= 1;
          three +=1;
        }
        else if array[i as usize] == 4 {
          three -= 1;
        }
      }
      if three > 0 { total_three += 1;}
      if two > 0 { total_two += 1; }
    }

  Ok(total_three*total_two)
}


use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::str::FromStr;
use std::collections::HashSet;

pub fn day1_part1(filename : &str) -> io::Result<i32> {

  let mut freq = 0;

  let f = File::open(filename)?;
  let f = BufReader::new(f);
  for line in f.lines() 
  {
   let num = i32::from_str(&line.unwrap()).unwrap();
   freq = freq + num;
 }
 Ok(freq)
}

pub fn day1_part2(filename : &str) -> io::Result<i32> {

  let mut freqs = HashSet::new();
  let mut freq = 0;

  loop {
    {
      let f = File::open(filename)?;
      let f = BufReader::new(f);
      for line in f.lines() {
       let num = i32::from_str(&line.unwrap()).unwrap();
       freq = freq + num;
       if !freqs.contains(&freq) {
        freqs.insert(freq);
      }
      else
      {
        return Ok(freq);
      }
    }
  }

}
}
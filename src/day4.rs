use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

pub fn day4_part1(filename : &str) -> io::Result<i32> {

  let f = File::open(filename)?;
  let f = BufReader::new(f);
  for line in f.lines() 
  {
  	println!("{}",line.unwrap());
  }
  Ok(0)
}

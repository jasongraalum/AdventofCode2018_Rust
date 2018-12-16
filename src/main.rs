use crate::day1::day1_part1;
use crate::day2::day2_part1;
use crate::day1::day1_part2;
use crate::day2::day2_part2;
use crate::day3::day3_part1;
use crate::day3::day3_part2;
use crate::day4::day4_part1;

use std::io;

mod day1;
mod day2;
mod day3;
mod day4;
mod xypair;


fn main() -> io::Result<()>  {

	let day1_filename = String::from("./data/day1_input.txt");
	let day2_filename = String::from("./data/day2_input.txt");
  let day3_filename = String::from("./data/day3_input.txt");
  let day4_filename = String::from("./data/day4_input.txt");

	println!("Day 1 - Part 1 = {}",  day1_part1(&day1_filename).unwrap());
	println!("Day 1 - Part 2 = {}",  day1_part2(&day1_filename).unwrap());
	println!("Day 2 - Part 1 = {}", day2_part1(&day2_filename).unwrap());
  println!("Day 2 - Part 2 = {}", day2_part2(&day2_filename).unwrap());
  println!("Day 3 - Part 1 = {}", day3_part1(&day3_filename).unwrap());
  println!("Day 3 - Part 2 = {}", day3_part2(&day3_filename).unwrap());
  println!("Day 4 - Part 1 = {}", day4_part1(&day4_filename).unwrap());
	Ok(())
}



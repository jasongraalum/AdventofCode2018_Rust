mod day1;
mod day2;
mod day3;
mod day4;
mod xypair;

use crate::day1::*;
use crate::day2::*;
use crate::day3::*;
use crate::day4::*;
use std::io;
use std::env;



fn main() -> io::Result<()>  {

	for arg in env::args() {
		if arg == "1" { 
		  let day_filename = String::from("./data/day1_input.txt");
			println!("Day 1 - Part 1 = {}",  day1_part1(&day_filename).unwrap());
			println!("Day 1 - Part 2 = {}",  day1_part2(&day_filename).unwrap());
		} else
		if arg == "2" { 
		  let day_filename = String::from("./data/day2_input.txt");
			println!("Day 2 - Part 1 = {}",  day2_part1(&day_filename).unwrap());
			println!("Day 2 - Part 2 = {}",  day2_part2(&day_filename).unwrap());
		} else 
		if arg == "3" { 
		  let day_filename = String::from("./data/day3_input.txt");
			println!("Day 3 - Part 1 = {}",  day3_part1(&day_filename).unwrap());
			println!("Day 3 - Part 2 = {}",  day3_part2(&day_filename).unwrap());
		} else 
		if arg == "4" { 
		  let day_filename = String::from("./data/day4_input.txt");
			println!("Day 4 - Part 1 = {}",  day4_part1(&day_filename).unwrap());
			//println!("Day 4 - Part 2 = {}",  day4_part2(&day_filename).unwrap());
		}								
	}
	Ok(())
}



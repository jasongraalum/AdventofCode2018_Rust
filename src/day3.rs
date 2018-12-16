use crate::xypair::XYPair;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::BTreeMap;

pub fn day3_part2(filename: &str) -> io::Result<usize> {
  let f = File::open(filename)?;
  let f = BufReader::new(f);

  let mut fabric : BTreeMap<XYPair,usize> = BTreeMap::new();

  for ll in f.lines()
  {
    let l = ll.unwrap().replace("#","").replace(":","").replace("x"," ").replace("@","").replace(","," ");
    let mut fields = l.split_whitespace();

    let _claim_number = fields.next().unwrap();
    let left = fields.next().unwrap().parse::<usize>().unwrap();
    let top = fields.next().unwrap().parse::<usize>().unwrap();
    let x = fields.next().unwrap().parse::<usize>().unwrap();
    let y = fields.next().unwrap().parse::<usize>().unwrap();

    for x_index in left..left+x {
      for y_index in top..top+y {
        let new_pair = XYPair{ x: x_index, y: y_index};
        let mut val : usize = 1;
        if fabric.contains_key(&new_pair) {
          val = fabric.get(&new_pair).unwrap() + 1;
        }
        fabric.insert(new_pair,val);
      }
    }
  }

  let f = File::open(filename)?;
  let f = BufReader::new(f);

  for ll in f.lines()
  {
    let l = ll.unwrap().replace("#","").replace(":","").replace("x"," ").replace("@","").replace(","," ");
    let mut fields = l.split_whitespace();

    let claim_number = fields.next().unwrap().parse::<usize>().unwrap();
    let left = fields.next().unwrap().parse::<usize>().unwrap();
    let top = fields.next().unwrap().parse::<usize>().unwrap();
    let x = fields.next().unwrap().parse::<usize>().unwrap();
    let y = fields.next().unwrap().parse::<usize>().unwrap();

    let mut no_overlap : bool = true;
    for x_index in left..left+x {
      for y_index in top..top+y {
        let new_pair = XYPair{ x: x_index, y: y_index};
        if fabric.contains_key(&new_pair) {
          let val = fabric.get(&new_pair).unwrap();
          if val > &1 { no_overlap = false };
        }
      }
    } 
    if no_overlap == true { return Ok(claim_number); }
  }   
  Ok(0)

}

pub fn day3_part1(filename: &str) -> io::Result<usize> {
  let f = File::open(filename)?;
  let f = BufReader::new(f);

  let mut fabric : BTreeMap<XYPair,usize> = BTreeMap::new();
  let mut count : usize = 0;

  for ll in f.lines()
  {
    let l = ll.unwrap().replace("#","").replace(":","").replace("x"," ").replace("@","").replace(","," ");
    let mut fields = l.split_whitespace();

    let _claim_number = fields.next().unwrap();
    let left = fields.next().unwrap().parse::<usize>().unwrap();
    let top = fields.next().unwrap().parse::<usize>().unwrap();
    let x = fields.next().unwrap().parse::<usize>().unwrap();
    let y = fields.next().unwrap().parse::<usize>().unwrap();

    for x_index in left..left+x {
      for y_index in top..top+y {
        let new_pair = XYPair{ x: x_index, y: y_index};
        let mut val : usize = 1;
        if fabric.contains_key(&new_pair) {
          val = fabric.get(&new_pair).unwrap() + 1;
          if val == 2 { count += 1; }
        }
        fabric.insert(new_pair,val);
      }
    }
  }

  Ok(count)

}


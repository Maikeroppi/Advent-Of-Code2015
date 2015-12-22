extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use regex::Regex;

#[derive(PartialEq)]
enum OpType
{
	AND,
	OR,
	LSHIFT,
	RSHIFT,
	NOT,
}

struct LogicSimulator
{ 
	evaluated:bool,
	name:String,
	value:u16,
	left_identifier:String,
	right_identifier:String,
}

impl Default for LogicSimulator {
  fn default () -> LogicSimulator 
  {
    LogicSimulator {evaluated : false, 
    	name: String::new(), 
    	value: 0, 
    	left_identifier: String::new(), 
    	right_identifier: String::new()
    }
  }
}

fn parse_line(in_str:&str)
{
	let number_regex = Regex::new(r"[0-9]+").unwrap(); 
	let tokens:Vec<&str> = in_str.split(' ').collect();
	if number_regex.is_match(tokens[0])
	{
		
	}
	else
	{
		
	}
}

fn main() 
{
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    // Iterate over lines
    for line in reader.lines()
    {
    	let unwrapped_line = line.unwrap();
    }
}

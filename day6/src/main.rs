use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[derive(PartialEq)]
enum SwitchType
{
	TurnOn,
	TurnOff,
	Toggle,
}

const TOGGLE_STR: &'static str = "toggle";
const TURN_ON_STR: &'static str = "turn on";
const TURN_OFF_STR: &'static str = "turn off";

fn parse(in_string:&str, out_type: &mut SwitchType, start_x: &mut u32, start_y: &mut u32, end_x: &mut u32, end_y: &mut u32)
{
	let mut start_str_length = 0;
	if in_string.starts_with(TOGGLE_STR)
	{
		*out_type = SwitchType::Toggle;
		start_str_length = TOGGLE_STR.len() + 1;
	}
	else if in_string.starts_with(TURN_ON_STR)
	{
		*out_type = SwitchType::TurnOn;
		start_str_length = TURN_ON_STR.len() + 1;
	}
	else if in_string.starts_with(TURN_OFF_STR)
	{
		*out_type = SwitchType::TurnOff;
		start_str_length = TURN_OFF_STR.len() + 1;
	}
	
	let start_end_str = &in_string[start_str_length..];
	let tokens:Vec<&str> = start_end_str.split_whitespace().collect();
	
	let start_pair:Vec<&str> = tokens[0].split(',').collect();
	let end_pair:Vec<&str> = tokens[2].split(',').collect();
	
	*start_x = start_pair[0].trim().parse().ok().expect("Start X");
	*start_y = start_pair[1].trim().parse().ok().expect("Start Y");
	
	*end_x = end_pair[0].trim().parse().ok().expect("End X");
	*end_y = end_pair[1].trim().parse().ok().expect("End Y");
}

#[cfg(test)]
fn test_parse(in_string:&str, expected_type:&SwitchType, start_x:u32, start_y:u32, end_x:u32, end_y:u32) -> bool
{
	let mut out_type = SwitchType::Toggle;
	let mut out_start_x: u32 = 0;
	let mut out_start_y: u32 = 0;
	let mut out_end_x: u32 = 0;
	let mut out_end_y: u32 = 0;
	
	parse(&in_string, &mut out_type, &mut out_start_x, &mut out_start_y, &mut out_end_x, &mut out_end_y);
	
	let correct = out_type == *expected_type && start_x == out_start_x && out_start_y == start_y && out_end_x == end_x && out_end_y == end_y;  

	println!("{}: correct: {}", in_string, correct);
	correct	
}

#[test]
fn run_string_parse_tests()
{
	assert_eq!(test_parse("turn on 489,959 through 759,964",	&SwitchType::TurnOn, 	489, 959, 759, 964), true);
	assert_eq!(test_parse("turn off 820,516 through 871,914",	&SwitchType::TurnOff, 	820, 516, 871, 914), true);
	assert_eq!(test_parse("toggle 756,965 through 812,992",		&SwitchType::Toggle,	756, 965, 812, 992), true);
}

fn main() 
{
	let mut lights = [[false; 1000]; 1000];
	let mut light_map: Vec< Vec<u32> > = Vec::new();;
	
	let mut start_x: u32 = 0;
	let mut end_x: u32 = 0;
	let mut start_y: u32 = 0;
	let mut end_y: u32 = 0;
	
	let mut out_type:SwitchType = SwitchType::Toggle;
	
	for i in 0..1000
	{
		let mut vec: Vec<u32> = Vec::new();
		for j in 0.. 1000
		{
			vec.push(0);
		}
		light_map.push(vec);
	}
	
	let f = File::open("day6input.txt").unwrap();
    let reader = BufReader::new(f);

    // Iterate over lines
    for line in reader.lines()
    {
    	let unwrapped_line = line.unwrap();
    	parse(&unwrapped_line, &mut out_type, &mut start_x, &mut start_y, &mut end_x, &mut end_y);
    	for i in start_y .. end_y + 1
		{
			for j in start_x .. end_x + 1
			{
				let r = i as usize;
				let c = j as usize;
				let value = lights[r][c];
				let brightness = light_map[r][c];
				if out_type == SwitchType::TurnOn
				{
					lights[r][c] = true;
					light_map[r][c] += 1;
				}
				else if out_type == SwitchType::TurnOff
				{
					lights[r][c] = false;
					if brightness > 0
					{
						light_map[r][c] -= 1;
					}
				}
				else if out_type == SwitchType::Toggle
				{
					lights[r][c] = !value;
					light_map[r][c] += 2;
				}
			}
		}
    }
	
	let mut light_count = 0;
	let mut total_brightness = 0;
	for i in 0 .. 1000
	{
		for j in 0 .. 1000
		{
			if lights[i][j]
			{
				light_count += 1;
			}
			
			total_brightness += light_map[i][j];
		}
	}
	
	println!("Part 1 - Light Count: {}", light_count);
	println!("Part 2 - Total Brightness: {}", total_brightness); 
}

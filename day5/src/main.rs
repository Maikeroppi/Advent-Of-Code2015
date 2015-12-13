use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn has_bad_string_types(test_string:&str) -> bool
{
	let mut contains_bad_string = false;
	if test_string.contains("ab") {
		contains_bad_string = true;
	} else if test_string.contains("cd") {
		contains_bad_string = true;
	} else if test_string.contains("pq") {
		contains_bad_string = true;
	} else if test_string.contains("xy") {
		contains_bad_string = true;
	}

	return contains_bad_string;
}

fn has_str_count(test_string:&str) -> bool
{
	let mut vowel_count:u32 = 0;
	let mut last_char:char = '0';
	let mut has_double_char = false;
	let mut return_val = false;
	for c in test_string.chars()
	{
		if last_char == c {
			has_double_char = true;
		}
		last_char = c;
		
		vowel_count += match c {
			'a' | 'e' | 'i' | 'o' | 'u' => 1,
			_ => 0,
		};
		
		return_val = has_double_char && vowel_count >= 3;
		if return_val {
			break;
		}
	}
	
	return_val
}

fn is_good_string1(test_string:&str) -> bool
{
	!has_bad_string_types(&test_string) && has_str_count(test_string)
}

fn do_test1(test_string:&str, should_be_nice:bool)
{
	let is_nice = is_good_string1(&test_string);
	let type_string = if is_nice { "nice" } else { "naughty" };
	let correct_string = if should_be_nice == is_nice { "CORRECT" } else { "INCORRECT" };
	println!("{} is {}: {}", test_string, type_string, correct_string);
}

fn has_double_letter(test_string:&str) -> bool
{
	let mut return_value = false;
	let test_size = test_string.len();
	let char_vec:Vec<char> = test_string.chars().collect();

	let mut first_index = 0;
	let mut second_index = 2;
	
	while second_index < test_size && !return_value
	{
		if char_vec[first_index] == char_vec[second_index]
		{
			return_value = true;
		}
		else
		{
			first_index += 1;
			second_index += 1;
		}
	}
	
	return return_value;
}

fn has_two_char_substring(test_string:&str) -> bool
{
	let mut return_value = false;
	let test_size = test_string.len();
	
	let mut index = 1;
	
	while index < test_size && !return_value
	{
		let search_str = &test_string[index-1..index+1];
		let sub_str = &test_string[index+1..];
		//println!("Sub: {}, Search: {}", sub_str, search_str);
		if sub_str.find(search_str) != None
		{
			println!("Str: {}, sub_str: {}", test_string, search_str);
			return_value = true;
		}
		else
		{
			index += 1;
		}
	}
	
	return return_value;
}

fn is_good_string2(test_string:&str) -> bool
{
	has_double_letter(test_string) && has_two_char_substring(test_string)
}

fn do_test2(test_string:&str, should_be_nice:bool)
{
	let is_nice = is_good_string2(&test_string);
	let type_string = if is_nice { "nice" } else { "naughty" };
	let correct_string = if should_be_nice == is_nice { "CORRECT" } else { "INCORRECT" };
	println!("{} is {}: {}", test_string, type_string, correct_string);
}

fn main() 
{
	if false {
		// Test first nice testing
	    do_test1("ugknbfddgicrmopn", true);
	    do_test1("aaa", true);
	    do_test1("jchzalrnumimnmhp", false);
	    do_test1("haegwjzuvuyypxyu", false);
	    do_test1("dvszwmarrgswjxmb", false);
	}
	
	if true {
		// Test second nice testing
		do_test2("qjhvhtzxzqqjkmpb", true);
		do_test2("xxyxx", true);
		do_test2("uurcxstgmygtbstg", false);
		do_test2("ieodomkazucvgmuy", false);
	}

	let f = File::open("day5input.txt").unwrap();
    let reader = BufReader::new(f);
    
    let mut nice_count1 = 0;
    let mut nice_count2 = 0;

    // Iterate over lines
    for line in reader.lines() {
    	let unwrapped_line = line.unwrap();
    	if is_good_string1(&unwrapped_line) {
    		nice_count1 += 1;
    	}
    	
    	if is_good_string2(&unwrapped_line) {
    		nice_count2 += 1;
    	}
    }
    
    println!("Number of Nice Words for first problem is {}", nice_count1);
    println!("Number of Nice Words for second problem is {}", nice_count2);
}

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

fn is_good_string(test_string:&str) -> bool
{
	!has_bad_string_types(&test_string) && has_str_count(test_string)
}

fn do_test(test_string:&str, should_be_nice:bool)
{
	let is_nice = is_good_string(&test_string);
	let type_string = if is_good_string(&test_string) { "nice" } else { "naughty" };
	let correct_string = if should_be_nice == is_nice { "CORRECT" } else { "INCORRECT" };
	println!("{} is {}: {}", test_string, type_string, correct_string);
}

fn main() 
{
	if false {
	    do_test("ugknbfddgicrmopn", true);
	    do_test("aaa", true);
	    do_test("jchzalrnumimnmhp", false);
	    do_test("haegwjzuvuyypxyu", false);
	    do_test("dvszwmarrgswjxmb", false);
	}

	let f = File::open("day5input.txt").unwrap();
    let reader = BufReader::new(f);
    
    let mut nice_count = 0;

    // Iterate over lines
    for line in reader.lines() {
    	if is_good_string(&line.unwrap()) {
    		nice_count += 1;
    	}
    }
    
    println!("Number of Nice Words is {}", nice_count);
}
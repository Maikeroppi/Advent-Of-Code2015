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

fn is_good_string2(test_string:&str) -> bool
{
	false
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

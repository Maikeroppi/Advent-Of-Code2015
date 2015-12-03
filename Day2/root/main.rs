use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use std::cmp;

fn get_area_from_line(line: &String) -> u32
{
    let number_strs:Vec<&str> = line.split('x').collect();
    let l:u32 = number_strs[0].trim().parse().ok().expect("Number1");
    let w:u32 = number_strs[1].trim().parse().ok().expect("Number2");
    let h:u32 = number_strs[2].trim().parse().ok().expect("Number3");
    
    let area1 = l * w;
    let area2 = w * h;
    let area3 = h * l;
    let smallest_area = cmp::min(cmp::min(area1, area2), cmp::min(area2, area3));
    //println!("Areas 1: {}, 2: {}, 3: {}, Min: {}", area1, area2, area3, smallest_area);
    (area1 * 2) + (area2 * 2) + (area3 * 2) + smallest_area
}
    
fn process_file() -> Result<(), io::Error>
{
    let f = try!(File::open("Day2Problem1.txt"));
    let reader = BufReader::new(f);
    //let mut buffer = String::new();

    let mut total_area = 0;

    // Iterate over lines
    for line in reader.lines() {
        let plain_line:String = line.unwrap();
        let area = get_area_from_line(&plain_line);
        //println!("Area: {}", area); 
        total_area += area;
    }
    
    println!("Total Area: {}", total_area);

    Ok(())
}

fn main() {
   process_file().ok().expect("Error reading file");
}

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use std::cmp;

fn get_area(w: &u32, h: &u32, l: &u32) -> u32
{
    let area1 = l * w;
    let area2 = w * h;
    let area3 = h * l;
    let smallest_area = cmp::min(cmp::min(area1, area2), cmp::min(area2, area3));
    //println!("Areas 1: {}, 2: {}, 3: {}, Min: {}", area1, area2, area3, smallest_area);
    (area1 * 2) + (area2 * 2) + (area3 * 2) + smallest_area
}

fn get_ribbon_length(w: &u32, h: &u32, l: &u32) -> u32
{
    let mut vec:Vec<u32> = Vec::new();
    vec.push(*w);
    vec.push(*h);
    vec.push(*l);
    vec.sort();
    
    let val1 = vec[0];
    let val2 = vec[1];
    
    let bow_length = w * h * l;
    let ribbon_length = (val1 * 2) + (val2 * 2) + bow_length;
    
    ribbon_length
}

fn get_area_from_line(line: &String, area: &mut u32, ribbon_length: &mut u32)
{
    let number_strs:Vec<&str> = line.split('x').collect();
    let l:u32 = number_strs[0].trim().parse().ok().expect("Number1");
    let w:u32 = number_strs[1].trim().parse().ok().expect("Number2");
    let h:u32 = number_strs[2].trim().parse().ok().expect("Number3");
    
    *area = get_area(&w, &h, &l);
    *ribbon_length = get_ribbon_length(&w, &h, &l);
}
    
fn process_file() -> Result<(), io::Error>
{
    let f = try!(File::open("Day2Problem1.txt"));
    let reader = BufReader::new(f);
    //let mut buffer = String::new();

    let mut total_area = 0;
    let mut total_ribbon_length = 0;

    // Iterate over lines
    for line in reader.lines() {
        let plain_line:String = line.unwrap();
        let mut area = 0;
        let mut ribbon_length = 0;
        get_area_from_line(&plain_line, &mut area, &mut ribbon_length);
        //println!("Area: {}", area); 
        total_area += area;
        total_ribbon_length += ribbon_length;
    }
    
    println!("Total Area: {}", total_area);
    println!("Total Ribbon Length: {}", total_ribbon_length);

    Ok(())
}

fn main() {
   process_file().ok().expect("Error reading file");
}

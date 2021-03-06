extern crate md5;

fn make_string(prefix:&String, value: &u32)->String
{
    let value_string = value.to_string();
    format!("{}{}", prefix, value_string)
}

fn digest_to_string(data:&md5::Digest, upper_byte:&mut u64, lower_byte:&mut u64) -> String
{
    *upper_byte = 0;
	*lower_byte = 0;
	
	for i in (8..16)
	{
		*lower_byte = *lower_byte << 8 | data[i] as u64;
	}
	
	 for i in (0..8)
    {
        *upper_byte = *upper_byte << 8 | data[i] as u64;
    }
	
	format!("{:016x}{:016x}", upper_byte, lower_byte)
}

fn main() 
{
    println!("Program running!");
    let prefix:String = "iwrupvqb".to_string();
        
    let mut current_value:u32 = 0;
    loop
    {
        let string_data = make_string(&prefix, &current_value);
        //println!("String data: {}", string_data);
        let digest = md5::compute(string_data.as_bytes());
		let mut upper_byte = 0;
		let mut lower_byte = 0;
        let digest_as_string = digest_to_string(&digest, &mut upper_byte, &mut lower_byte);
		//println!("Digest as string: {}", digest_as_string);
		//assert_eq!(digest_as_string, "3ad2e81f76bd3a870f62a1907dfae973");
        
		if upper_byte & 0xfffff00000000000_u64 == 0
		{
			println!("5 zero Hash found!: {}", digest_as_string);
			println!("Value: {}", string_data);
		}
		
		if upper_byte & 0xffffff0000000000_u64 == 0
		{
			println!("6 zero Hash found!: {}", digest_as_string);
			println!("Value: {}", string_data);
			break;
		}
       
        current_value += 1;
    }
    
}

match my_str.parse::<i32>() {
	Ok(x) => {my_int = x;},
	Err(e) => {
		if e.to_string() == "invalid digit found in string" {
			println!("Only positive integers are allowed!");
		}
		else if e.to_string() == "number too large to fit in target type" {
			println!("Number is too large!");
		}
		else if e.to_string() == "number too small to fit in target type" {
			println!("Number is too small!");
		}
		else if e.to_string() == "cannot parse integer from empty string" {
			println!("Number is empty!");
		}
	}
}

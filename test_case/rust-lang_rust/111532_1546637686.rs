rust
fn main() {
	let bytes: &'static str;
	let file: Result<&str, &str> = Ok("File data");

	if let Err(_error) = file {
		std::process::exit(1);
	} else if let Ok(store) = file {
		bytes = store
	} else {
		// bytes uninitialized for this case
		// the compiler doesn't recognize this else block is impossible
	}
	
	println!("{}", bytes);
}


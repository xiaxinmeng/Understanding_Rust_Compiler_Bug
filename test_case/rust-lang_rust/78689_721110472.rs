rust
	let mut not_enough_space = [0u8; 10];
	let mut stream = BufWriter::new(not_enough_space.as_mut());
	write!(stream, "this does not fit entirely in the array").unwrap();
        ...

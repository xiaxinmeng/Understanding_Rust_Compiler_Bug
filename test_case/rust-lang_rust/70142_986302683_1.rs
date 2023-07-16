rust
let future_with_flattened_result = async { 
	let dur = Duration::from_millis(100);
	let s = fs::read_to_string("./my.db").timeout(dur).await??;

	Ok(s)
};

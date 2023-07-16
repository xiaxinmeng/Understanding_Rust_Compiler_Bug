rust
// b.rs
use std::env;
fn main() {
	let args: Vec<String> = env::args().collect();
	let x: u128 = match args[1].parse() {
		Ok(n) => {
			n
		},
		Err(_) => {
			eprintln!("No");
			return;
		},
	};
	let y = x.reverse_bits();
	println!("{} {}", x, y); 
}

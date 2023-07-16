
//#[derive(Clone)] //Uncomment and it will suddenly works
pub struct Example {
	pub name : String
}

fn main() {
	let ex = Example{name: "Name".to_string()};
	let ex_ref = &ex;
	let ex_owned : Example = ex_ref.to_owned();  // Error: expected struct `Example`, found &Example
}

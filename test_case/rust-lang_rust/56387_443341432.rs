rust

use std::ffi::OsStr;

fn main() {
	let s = OsStr::new("asdf");
	match s {
		OsStr::new("png") => {},
	};
}

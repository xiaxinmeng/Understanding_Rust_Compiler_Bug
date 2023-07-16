rust
#![feature(never_type)]

fn main() {
	unsafe {
		let a = std::mem::uninitialized::<!>();
	}
}


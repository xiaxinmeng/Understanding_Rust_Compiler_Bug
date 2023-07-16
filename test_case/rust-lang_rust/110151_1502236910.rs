rust
async fn empty() {}

async fn meow(a: [u8; 100]) {
	empty().await;
	dbg!(a);
}

fn main() {
	let v = meow([0; 100]);
	let e = [0u8; 202];
	let _ = || unsafe {
		use std::{mem, ptr};
		let mut copy = ptr::read(&v);
		ptr::write(&mut copy, mem::transmute(ptr::read(&e)));
		mem::forget(copy);
	};
}

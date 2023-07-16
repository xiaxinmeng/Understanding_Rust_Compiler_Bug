`rust
#![feature(intrinsics)]
extern "rust-intrinsic" {
	fn dstfn(dst: *mut dst, dst: dst);
}

fn main() {
	dstfn(1);
}

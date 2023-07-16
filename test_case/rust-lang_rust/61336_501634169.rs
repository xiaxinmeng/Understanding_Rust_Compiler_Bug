
#![feature(const_generics)]

fn f<T: Copy /* ICE's just as badly without Copy */, const N: usize>(x: T) -> [T; N] {
	[x; {N}]
}

fn main() {}

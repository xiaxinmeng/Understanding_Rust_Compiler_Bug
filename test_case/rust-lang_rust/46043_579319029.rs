rust
#[repr(packed)]
struct X {
    p: u8,
    q: u32
}

fn main() {
	let x = X { p: 0, q: 0 };

	let y: &u32 = &x.q;
}

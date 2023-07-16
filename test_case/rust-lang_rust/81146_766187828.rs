rust
#[no_mangle]
fn sum(x: [u8; 4], y: [u8; 4]) -> [u8; 4] {
	[x[0] + y[0], x[1] + y[1], x[2] + y[2], x[3] + y[3]]
}

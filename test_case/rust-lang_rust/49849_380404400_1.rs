
fn main() {
	let mut x: Vec<u8> = Vec::with_capacity(1_000_000_000);
	let mut y: Vec<u8> = Vec::with_capacity(x.capacity()); unsafe{y.set_len(x.capacity())};
	let time = std::time::Instant::now();
	for i in 0..x.capacity() {
		x.push(i as u8);
	}
	unsafe{memcpy(&mut x[0], &y[0], x.capacity())};
	println!("{:?}", time.elapsed());
}

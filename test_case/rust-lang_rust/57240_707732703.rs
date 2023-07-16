
const Q: u8 = b'q';
const fn ctrl_key(key: u8) -> u8 {
	key & 0x1f
}
const CTRL_Q: u8 = ctrl_key(Q);

fn main() {
	let result: Option<u8> = read_byte_from_somewhere();
	match result {
		Some(Q) | Some(ctrl_key(Q)) => return,
		Some(i) if i > 0 => println!("Read {} from somewhere", i),
		_ => return,
	}
}

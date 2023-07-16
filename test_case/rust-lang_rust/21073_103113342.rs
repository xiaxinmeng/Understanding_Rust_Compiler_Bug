 rust
const TAG: &'static [u8] = b"ABCD";

macro_rules! tag(
    ($value:expr) => (unsafe {
        let value: [u8; 4] = ::std::mem::transmute($value);
        &vec![value[3], value[2], value[1], value[0]][..]
    })
);

fn main() {
    match tag!(42) {
        TAG => println!("Yes."),
        _ => println!("No."),
    }
}

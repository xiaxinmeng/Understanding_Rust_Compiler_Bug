 rust
enum Eu64 {
    Au64 = 0,
    Bu64 = 0x8000_0000_0000_0001 //~WARN literal out of range for isize
}

fn main() {
    println!("{} {} {}", std::mem::size_of::<Eu64>(), Eu64::Au64 as isize, Eu64::Bu64 as isize);
}

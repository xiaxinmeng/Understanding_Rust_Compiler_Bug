rust
union TransmuteHack<T: Copy> {
    from: T,
    to: usize,
}

pub static VALUE: usize = 42;

fn main() {
    println!("{:?}", unsafe { TransmuteHack { from: &VALUE }.to } / 2);
}

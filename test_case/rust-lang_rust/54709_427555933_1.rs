rust
union TransmuteHack<T: Copy> {
    from: T,
    to: usize,
}

pub static VALUE: usize = 42;
pub static ADDRESS: usize = unsafe { TransmuteHack { from: &VALUE }.to };

fn main() {
    println!("{:?}", ADDRESS / 2);
}

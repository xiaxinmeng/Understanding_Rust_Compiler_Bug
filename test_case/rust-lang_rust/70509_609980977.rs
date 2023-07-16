rust
#![feature(repr128, arbitrary_enum_discriminant)]

#[derive(PartialEq, Debug)]
#[repr(i128)]
enum Test {
    A(Box<u64>) = 0,
    B(usize) = u64::max_value() as i128 + 1, 
}

fn main() {
    assert_eq!(Test::A(Box::new(2)), Test::B(0));
    //~^ Illegal instruction (core dumped)
}


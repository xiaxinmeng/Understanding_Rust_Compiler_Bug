 rust
#![feature(macro_rules)]

macro_rules! test
{
    ($a: expr $b: expr) =>
    {
        $a as u32 + $b as u32
    }
}

fn main()
{
    println!("{}", test!(0o19));
}

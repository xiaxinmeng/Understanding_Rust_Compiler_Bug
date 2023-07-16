 rust
use std::ops::Fn;

fn foo(i: int) -> int
{
    i
}

fn main()
{
    println!("{}", (&foo as &Fn<int, int>).call(1));
}

 rust
#![feature(unboxed_closures)]

fn main ()
{
    let f = |&:| { };
    let g = |&:| f.call(());
}

 rust
#![feature(unboxed_closures)]
#![feature(core)]

use std::cmp::Ordering;

pub fn comparing<F,T,B>(f: F) -> Comparing<F> where
    F: Fn(&T) -> B,
    B: Ord
{
    Comparing(f)
}

pub struct Comparing<F>(F);

impl<'a,'b,F,T,B> Fn<(&'a T, &'b T)> for Comparing<F> where
    F: Fn(&T) -> B,
    B: Ord
{
    type Output = Ordering;

    extern "rust-call" fn call(&self, args: (&T, &T)) -> Ordering {
        self.0(args.0).cmp(&self.0(args.1))
    }
}

#[derive(Debug)]
struct A(u8);

fn main() {
    let mut a = [A(4), A(2), A(6), A(9)];
    a.sort_by(comparing(|x: &A| x.0));
    println!("{:?}", a);
}

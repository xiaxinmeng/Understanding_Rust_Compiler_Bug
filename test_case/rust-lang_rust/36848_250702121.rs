 rust
#![feature(specialization)]

pub trait Iterator {
    type Item;

    fn next(&self) -> Option<Self::Item>;
}

impl<'a> Iterator for &'a () {
    type Item = &'a u32;

    fn next(&self) -> Option<&'a u32> { None }
}

pub struct Cloned<I>(I);

impl<'a, I, T: 'a> Iterator for Cloned<I>
    where I: Iterator<Item=&'a T>, T: Clone
{
    type Item = T;

    fn next(&self) -> Option<T> { None }
}

impl<'a, I, T: 'a> Iterator for Cloned<I>
    where I: Iterator<Item=&'a T>, T: Copy
{

}

fn main() {
    Cloned(&()).next();
}

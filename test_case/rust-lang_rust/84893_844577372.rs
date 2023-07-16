
#![allow(unreachable_code)]

pub trait MyHash {}

pub struct MyHashSet<T>(T);

impl<T> Eq for MyHashSet<T> where T: MyHash {}

impl<T> PartialEq for MyHashSet<T> where T: MyHash {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

#[derive(Eq)]
pub struct CustomSet<T>(MyHashSet<T>);

impl<T> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

fn main() {}

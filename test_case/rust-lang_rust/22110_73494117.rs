 rust
use std::slice::Iter;

fn foo<'a,T>(v: Iter<'a,T>) -> Option<T>
    where Iter<'a,T> : Iterator
{
    v.next()
}

fn main() { }

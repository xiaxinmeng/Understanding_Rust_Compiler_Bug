rust
#![feature(non_lifetime_binders)]

pub fn foo()
where
    for<V> V: Sized,
{
}

fn main() {
    foo();
}

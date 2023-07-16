rust
#![feature(generic_associated_types)]

trait Iterator {
    type Item<'this>: Get
    where
        Self: 'this;

    fn next(&mut self) -> Self::Item<'_>;
}

trait Get {
    type Value: 'static;

    fn get(self) -> Self::Value;
}

fn not_ok<T>(mut it: T)
where
    T: Iterator,
{
    let mut _out;

    loop {
        let item = it.next(); // `it` was mutably borrowed here in the previous iteration of the loop
        _out = item.get();
    }
}

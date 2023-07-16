rust
#![feature(never_type)]

impl Iterator for ! {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        None
    }
}

fn my_func() -> impl Iterator<Item = u32> {
    todo!()
}

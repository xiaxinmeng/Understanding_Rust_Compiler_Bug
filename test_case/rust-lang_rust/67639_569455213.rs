rust
fn main() {
    join_all::<u32>();
}

trait Foo {
    type Item;
}

impl Foo for u32 {
    type Item = u8;
}

trait Bar {
    type Item2;
}

impl Bar for u8 {
    type Item2 = u64;
}

fn join_all<I>()
where I: Foo,
    I::Item: Bar
{
    Vec::<<I::Item as Bar>::Item2>::new(); // ICE occurs processing this line
}

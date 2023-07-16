rust
fn main() {
    let a: Foo::Item;
}

struct Foo<Item = usize>{
    phan:std::marker::PhantomData<Item>,
}

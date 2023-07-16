 rust
trait Foo {
    type Ty;
}

impl Foo for () {
    type Ty = ();
    type Ty = usize;
}

fn main() { 
    let _: <() as Foo>::Ty = ();
}

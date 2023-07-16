rust
#![feature(never_type)]
trait Foo {
   fn recommended_quantity() -> usize;
}

fn do_thing<T: Foo>(initialize: bool, f: fn() -> T) -> Vec<T>  {
    let mut v = Vec::with_capacity(T::recommended_quantity());
    if initialize {
        v.resize_with(T::recommended_quantity(), f);
    }
    v
}

impl Foo for ! {
    fn recommended_quantity() -> usize {
        panic!("hypothetically, this implementation is implicit");
    }
}

fn main() {
    fn never() -> ! { loop {} }
    do_thing(false, never);
}

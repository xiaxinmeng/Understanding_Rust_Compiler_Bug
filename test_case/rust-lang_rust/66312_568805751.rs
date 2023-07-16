rust
#![feature(arbitrary_self_types)]
trait Test<T: core::ops::Deref<Target = Self>> {
    fn is_some(self: T);
}

fn f() {
    let x = Some(2);
    if x.is_some() {
        println!("Some");
    }
}

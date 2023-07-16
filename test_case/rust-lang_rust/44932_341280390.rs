rust
struct S {}

impl Trait for S {
    // Notice: &self, not *const self
    fn f(&self) -> &'static str {
        "S"
    }
}

fn main() {
    // Notice: &Trait, not a raw pointer
    let s: &'static Trait = unsafe { ptr::null::<S>().as_ref().unwrap() };
    println!("{}", s.f());
}

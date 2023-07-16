rust
struct S {}

impl Trait for S {
    unsafe fn f(*const self) -> &'static str {
        "S"
    }
}

fn main() {
    let s: *const Trait = ptr::null::<S>();
    println!("{}", s.is_null()); // false, calling Trait::f is fine

    let s: *const Trait = ptr::null::<Trait>();
    println!("{}", s.is_null()); // true, calling Trait::f is not fine
}

 rust
#![feature(unboxed_closures)]

fn foo<'a>() -> &'a u32 { loop { } }

fn bar<T>(t: T, x: T::Output) -> T::Output
    where T: FnOnce<()>
{
    x
}

fn main() {
    let x = &22;
    let z: &'static u32 = bar(|| -> &u32 { loop { } }, x); // <-- Effectively casts stack pointer to static pointer
}

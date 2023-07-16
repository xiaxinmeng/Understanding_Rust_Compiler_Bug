rust
#![feature(unboxed_closures)]

fn foo<F: FnOnce<()>>()
{
    let _: Vec<F::Output> = vec![];
}

fn main() {
    foo::<fn() -> str>()
}

rust
#![feature(non_lifetime_binders)]
#![crate_type = "lib"]

async fn walk2<'a, T: 'a>(_: T)
where
    for<F> F: 'a,
{}

fn main() {}

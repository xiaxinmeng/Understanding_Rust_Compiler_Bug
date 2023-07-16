rust
fn foo<#[rustc_synthetic] T>(_: T) {
}

fn main() {
    let x = foo::<u32>; //~ ERROR cannot provide explicit type parameters
    foo(22); // ok
}

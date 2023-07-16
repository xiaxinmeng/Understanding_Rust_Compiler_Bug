 rust
fn foo<F, X>(cb: F) -> X
    where F : for<'a> FnOnce(&'a u64) -> X {
    foo(move |expr1| {
        cb(expr1)
    })
}

fn main() {
    foo(move |_| ());
}

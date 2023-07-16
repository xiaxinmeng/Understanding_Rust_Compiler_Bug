 rust
fn foo<F1, X, F2>(f1: F1, f2: F2)
    where F1: FnOnce() -> X,
          F2: Fn(&mut X)
{
    let mut x = f1();
    f2(&mut x);
}

fn main() {
    let a = || 0 as u32;
    let b = |_| {};
    foo(a, b);
}

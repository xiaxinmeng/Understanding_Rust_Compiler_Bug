 rust
fn caller<F>(f: F)
    where F: Fn(&i32)
{
    f(&42);
}

fn main() {
    // Works
    caller(|a| println!("{}", a));

    // Doesn't work
    let f = |a| println!("{}", a);
    caller(f);
}

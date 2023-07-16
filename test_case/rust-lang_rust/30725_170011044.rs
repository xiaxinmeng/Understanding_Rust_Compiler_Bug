 rust
fn foo(mut x: (Option<usize>, &str)) {
    println!("x  pre: {:?}", x);
    if let (ref mut y @ None, s) = x {
        *y = Some(s.len());
    }
    println!("x post: {:?}", x);
}

fn main() {
    foo((None, "hi"));
    foo((Some(10), "hello"));
}

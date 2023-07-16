 Rust
#[derive(Clone)]
struct S;

fn borrow<'a>(s: &'a S) -> Vec<&'a S> { vec![s] }

fn main() {
    let s = S;
    match <Vec<&S> as IntoIterator>::into_iter(borrow(&s)) {
        _ => drop(s)
    };
}

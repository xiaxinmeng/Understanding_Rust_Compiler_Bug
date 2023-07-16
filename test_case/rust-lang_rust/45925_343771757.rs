rust
fn foo(s: &str) -> bool { true }
fn main() {
    let x = vec![(String::new(), String::new())];
    x.iter()
        .filter(|&(ref a, _)| foo(a))
        .collect();
}

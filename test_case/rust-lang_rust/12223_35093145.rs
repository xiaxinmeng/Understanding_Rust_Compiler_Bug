 rust
fn main() {
    let a = ~"";
    let b: ~[&str] = a.lines().collect();
    drop(a);
    for s in b.iter() {
        println!("{}", *s);
    }
}

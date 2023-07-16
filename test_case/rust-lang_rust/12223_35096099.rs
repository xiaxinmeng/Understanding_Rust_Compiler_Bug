 rust
fn main() {
    let a = ~"";
    let b = a.lines().to_owned_vec();
    drop(a);
    for s in b.iter() {
        println!("{}", *s);
    }
}

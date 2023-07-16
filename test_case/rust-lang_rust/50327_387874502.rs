
fn main() {
    struct K;
    impl Drop for K { fn drop(&mut self) { println!("K dropped"); } }
    struct M {k: K}
    let m = Some(M{k: K});
    if let Some(M{k}) = m { }
    println!("hello");
}

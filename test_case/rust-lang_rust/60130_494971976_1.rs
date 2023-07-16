rust
fn main() {
    let mut iter = [1, 2, 3, 4, 5].iter().inspect(|_| { println!("next()") }).rev();
    
    let _ = iter.last();
}

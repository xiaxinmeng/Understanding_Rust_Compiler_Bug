 rust
fn main() {
    let x = [box 1i, box 2i];
    for &a in x.iter() {
        println!("{}", a);
    }
    for &a in x.iter() {
        println!("{}", a);
    }
}

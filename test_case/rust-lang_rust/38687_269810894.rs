rust
fn main() {
    let a = [0; 10000000];
    for i in a.iter() {
        println!("{}", i);
    }
}

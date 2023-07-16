rust
// data.rs
fn main() {
    let data  = [0, 1, 2, 3];
    for i in 0..data.len() {
        if data[i] == 0 {
             println!("{}", i - 1); 
        }
    }
}

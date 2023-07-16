 rust
fn main() {
    let i : i64 = 5 ;
    if i as usize < std::mem::size_of::<i64>() {
        println!("Case A");
    }
    if i as usize < 5 {
        println!("Case B");
    }
}

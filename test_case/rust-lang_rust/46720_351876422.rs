Rust
fn main() {
    let x = 0;
    match x {
        _ => {
            4 //~ ERROR
        }
    } // (there's a missing semicolon here)
    println!("hi!");
}

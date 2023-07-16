rust
fn main() {
    let _ = if false {
        Ok(panic!())
    } else {
        Err("")
    };
}

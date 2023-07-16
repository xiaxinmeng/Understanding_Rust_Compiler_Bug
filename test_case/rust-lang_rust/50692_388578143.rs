rust
fn main() {
    let array = [0; 16];
    let mut option = None;
    if let Some(n) = option {
        array[n];
        array[n].to_le();
    }
}

rust
fn main() {
    let mut x = 0;
    || {
        || {
            let _y = &mut x;
        }
    };
}

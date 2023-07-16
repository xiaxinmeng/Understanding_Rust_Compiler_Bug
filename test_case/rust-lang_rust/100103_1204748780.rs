rust
fn main() {
    let Some(x) = Some(()) else {
        match Err(()) {
            Err(()) => return (),
            Ok(val) => val,
        }
    };
}

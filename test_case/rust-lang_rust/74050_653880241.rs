rust
fn main() {
    let _arg = match Some(()) {
        Some(arg) => {
            match Some(&arg) {
                Some(s) => s,
                None => return,
            }
        } // <- this is probably the desired location
        None => return,
    };
}

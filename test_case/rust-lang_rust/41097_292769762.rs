rust
fn main() {
    let mut start = 0;
    let end = 10000;
    loop {
        let tag;
        if start < end {
            start += 1;
            tag = true;
        } else {
            tag = false;
        }
        if !tag { break; }
    }
}

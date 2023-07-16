 rust
fn main() {
    let s = "find α-particles";
    let mut start = s.len();
    for (i, c) in s.char_indices() {
        if c == 'p' {
            start = i;
            break;
        }
    }
    println!("remaining={}", s.slice_from(start));
}

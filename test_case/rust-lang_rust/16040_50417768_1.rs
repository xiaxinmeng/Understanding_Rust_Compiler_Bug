 rust
fn replace_last<T, S: MutableSeq<T>>(seq: &mut S, val: T) {
    if seq.pop().is_some() {
        seq.push(val)
    }
}

fn main() {
    let mut s = String::from_str("this works?");
    replace_last(&mut StringSeq(&mut s), '!');
    assert_eq!(s.as_slice(), "this works!");
}

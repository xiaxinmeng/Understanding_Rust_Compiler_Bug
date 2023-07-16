rust
fn main() {
    let mut iter = vec![1, 2, 3_u8].into_iter();

    let mut v: Vec<u8> = vec![];

    while let Some(_) = iter.next() {
        v.last_mut().unwrap() = 3_u8;
    }
}

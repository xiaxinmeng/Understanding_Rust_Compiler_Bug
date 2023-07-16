rust
fn iter_len<
    I: IntoIterator<IntoIter = Iter, Item = Item>,
    Iter: Iterator<Item = Item> + ExactSizeIterator,
    Item,
>(
    iter: I,
) {
    println!("size {}", iter.into_iter().len());
}

fn main() {
    let v: Vec<u8> = vec![1, 2, 3];
    iter_len(v.iter()); // ok
    iter_len(v);
}

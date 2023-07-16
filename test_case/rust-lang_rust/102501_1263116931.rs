rust
fn main() {
    let v: Vec<u32> = vec![1,2,3,4];

    println!("{:?}", (&v).into_iter().position(|x| x == &2)); // Some(1)
    // It is common for a types to impl pub fn iter(&self) -> SomeIterator<'_, T>
    // as an iterator over &T, so another way to spell the above
    println!("{:?}", v.iter().position(|x| x == &2)); // Some(1)
    println!("{:?}", v); // [1, 2, 3, 4] with no clones required
}

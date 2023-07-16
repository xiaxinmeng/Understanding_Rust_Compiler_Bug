 rust
fn main() {
    let mut data = vec![1; 100];
    {
        let (left, right) = DerefMut::deref_mut(&mut data).split_at_mut(Deref::deref(&data).len() / 2);
    }
    println!("{:?}", data);
}

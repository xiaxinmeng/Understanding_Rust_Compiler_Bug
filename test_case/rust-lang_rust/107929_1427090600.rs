rust
fn main() {
    let mut stuff = vec!(vec!(1));
    for i in 0..50000 {
        let len = (stuff[i].len() * 134775813) % 4096;
        stuff.push((1234123414u32..).take(len).collect());
    }
    std::mem::drop(stuff);
    let _ = vec!((Box::new(()), vec![[0u64; 8]])).as_slice().to_owned();
}

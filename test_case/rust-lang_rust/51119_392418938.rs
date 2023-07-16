rust
fn main() {
    let mut iter = ["0", "a", "1", "2"].iter().map(|x| x.parse::<i32>().ok());

    for _ in 0..5 {
        println!("{:?}", iter.next());
    }
    // => Some(Some(0)), Some(None), Some(Some(1)), Some(Some(2)), None
}

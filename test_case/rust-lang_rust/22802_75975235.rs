 rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let mut it = a.iter();
    {
        let mut it2 = it.by_ref().peekable();
        let _ = it2.peek();
    }
    for i in it {
        println!("{}", i);
    }
}

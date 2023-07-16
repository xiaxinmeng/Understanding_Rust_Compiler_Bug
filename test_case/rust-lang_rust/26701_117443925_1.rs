 rust
struct Test<'a> {
    n: usize,
    r: Option<&'a Test<'a>>,
}

impl<'a> Drop for Test<'a> {
    fn drop(&mut self) {
        println!("dropping {}", self.n);
    }
}

fn main() {
    println!("Hello, world!");
    let a = Test { n: 1, r: None};
    println!("Hello, world 2.0!");
    let a = Test { n: 2, r: Some(&a) };
    println!("Hello, world 3.0!");
}

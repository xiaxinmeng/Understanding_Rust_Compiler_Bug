 rust
struct Bar<'a> {
    i: i32,
    j: Option<&'a i32>,
}

impl <'a> Bar<'a> {
    fn new() -> Bar<'a> {
        Bar { i: 0, j: None }
    }

    fn set(&'a mut self, i: i32) {
        self.i = i;
        self.j = Some(&self.i);
    }
}

fn main() {
    let mut bar = Bar::new();
    bar.set(1);
    let j: &i32 = bar.j.unwrap();
    println!("{}", j);
    bar.i = 2;
    println!("{}", j);
}

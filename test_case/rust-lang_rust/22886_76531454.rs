 rust
fn crash_please() {
    let mut iter = Newtype {
        option: Some(Box::new(0)),
        marker: (),
    };

    let saved = iter.next().unwrap();
    println!("{}", saved);
    iter.option = None;
    println!("{}", saved);
}

struct Newtype<'a, T: 'a> {
    option: Option<Box<usize>>,
    marker: T,
}

impl<'a, T> Iterator for Newtype<'a, T> {
    type Item = &'a Box<usize>;

    fn next(&mut self) -> Option<&Box<usize>> {
        self.option.as_ref()
    }
}

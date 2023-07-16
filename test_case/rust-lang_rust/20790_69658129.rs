 rust
struct MyStruct {
    x: isize,
    y: isize,
}

impl MyStruct {
    fn next(&mut self) -> Option<isize> {
        Option::Some(self.x)
    }
}

fn main() {
    let mut bogus = MyStruct {
        x: 1,
        y: 2,
    };
    for x in bogus {
//~^  old-ERROR has type `MyStruct` which does not implement the `Iterator` trait
//~^^ new-ERROR the trait `iter::Iterator` is not implemented for the type `MyStruct`
    }
}

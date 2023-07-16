rust
enum Never { }

impl Never {
    fn foo(self) {
        match self { }
    }
}

fn main() { }

 rust
struct Foo { a: int }

impl Drop for Foo {
    fn drop(&mut self) {
        println!("{}", self.a);
    }
}

fn main() {
    {
        let _1 = Foo { a: 1 };
        let _2 = Foo { a: 2 };
        match Foo { a: 3 } {
            _ => {}
        }
    }
    let _4 = Foo { a: 4 };
}

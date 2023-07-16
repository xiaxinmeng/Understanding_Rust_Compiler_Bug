rust
mod foo {
    type Bar = u32;
}

mod baz {
    use foo::Bar;

    fn baz() {
        let x: Bar = 22;
    }
}

fn main() { }

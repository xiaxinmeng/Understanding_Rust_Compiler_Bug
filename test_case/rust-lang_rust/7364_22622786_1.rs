 rust
enum Foo<T> {
    Call(@fn() -> T)
}

static f: Foo<()> = Call(main);

fn main() { }

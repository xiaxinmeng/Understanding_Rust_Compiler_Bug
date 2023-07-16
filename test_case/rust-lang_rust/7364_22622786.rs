 rust
enum Foo<F> {
    Call(F)
}

static f: Foo<@fn()> = Call(main);

fn main() { }

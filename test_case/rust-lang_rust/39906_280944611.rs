rust
enum Bar {
    Qux(usize),
    Zar(String, usize),
}

struct Foo {
    bar: usize,
}

struct X<T1, T2> {
    x: T1,
    y: T2,
}

fn foo() -> X<X<String, String>, String> {
    X{ x: X{x: "".to_string(), y: 2 as usize}, y: 3}
}

fn zar() -> Result<Option<Foo>, Bar> {
    Some(Foo { bar: 1 })
}

fn z() -> Result<Option<Result<Option<()>, ()>>, ()> {
    Some(())
}

fn main() {}

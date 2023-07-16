
struct Foo {
    a: i32
    b: i32,
}

struct Bar {
    a: i32,,
    b: i32,
}

fn main() {
    let x = Foo {
        a: 42
        b: 3,
    };
    let y = Bar {
        a: 42,,
        b: 3,
    };
}

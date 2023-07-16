rust
struct Foo {
    f: i32,
}

fn touch() {}

pub fn f() {
    let x = Foo { f: 10 };
    let mut y = 5;

    touch(); // Just want to separate basic blocks

    y = x.f;

    touch();

    Foo { f: y } = x;

    touch();
}

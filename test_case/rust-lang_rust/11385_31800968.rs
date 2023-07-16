
struct Foo<'a> {
    x: int,
    y: Option<&'a int>,
}

fn f() -> Foo {
    Foo {
        x: 1,
        y: None,
    }
}

fn g<'a>(x: &'a mut Foo<'a>) {
    x.y = Some(&x.x);
}

fn h() -> Foo {
    let mut x = f();
    g(&mut x);
    x
}

fn main() {
    let _x = h();
}

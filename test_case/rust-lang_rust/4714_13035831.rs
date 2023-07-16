 rust
fn f(arg: A) {
    arg.a = 100;
}

fn g(arg: &mut A) {
    arg.a = 100;
}

struct A { mut a: int }

pub fn main() {
    let mut x = A {a: 10};
    g(&mut x);
    assert x.a == 100;
    x.a = 20;
    f(copy x);
    assert x.a == 20;
}


struct A<'self> {
    a: &'self [~int],
    b: ~int,
}

fn main() {
    let x: ~[~int] = ~[];
    let _a = A {
        a: x,
        b: ~1,
    };
}

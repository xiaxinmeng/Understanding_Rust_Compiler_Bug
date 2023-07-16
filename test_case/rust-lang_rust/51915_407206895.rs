rust
struct S {
    a: &'static str,
}

fn f(_: &mut S, _: &str) {
}

fn main() {
    let mut s = S {
        a: "a",
    };

    let r = &mut s;

    f(r, r.a);
}

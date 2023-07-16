 rust
extern mod foo;

fn main() {
    let mut y = 2;
    let x = || {
        42 * 7 + y
    };
    y = 5;
    foo::bar(x);
}

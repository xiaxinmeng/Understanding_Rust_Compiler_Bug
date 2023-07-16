Rust
fn foo() {}
const FOO: fn() = foo;

fn bar() {}
const BAR: fn() = bar;

const fn hello() {
    match FOO {
        FOO => (),
        BAR => (),
        _ => unreachable!(),
    }
}

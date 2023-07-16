
fn foo(_: Fn() -> bool) {}

fn main() {
        foo(|| { true; });
}


fn f() -> impl Sized {}

fn main() {
    let x = || f();
}

 rust
fn foo<T>() -> T { unreachable!() }
pub struct X<T>(T);
pub struct Y<T>(T);

fn main() {
    let mut x = X(foo());
    x = Y(x);
}

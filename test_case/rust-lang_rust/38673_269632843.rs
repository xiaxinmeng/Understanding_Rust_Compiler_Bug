rust
fn foo<F, P>(f: F)
    where F: Fn(P),
          P: for<'a> From<&'a str>
{
    f(From::from("test"))
}

struct Unit;

impl<'a> From<&'a str> for Unit {
    fn from(_: &'a str) -> Unit {
        Unit
    }
}

fn main() {
    foo(|_: Unit| {
        // ...
    })
}

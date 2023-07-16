rust
fn foo<F, P: ?Sized>(f: F)
    where for<'a> F: Fn(&'a P),
          for<'a> &'a P: From<&'a str>
{
    f(From::from("test"))
}

fn main() {
    foo(|_: &str| {
        // ...
    })
}

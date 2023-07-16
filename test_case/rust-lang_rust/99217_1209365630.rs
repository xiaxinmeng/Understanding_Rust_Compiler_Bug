rust
trait WithProj {
    type Assoc;
}

impl<'a, T> WithProj for &'a T {
    type Assoc = T;
}

fn foo<'a, T>(_: <&'a T as WithProj>::Assoc) -> &'a () { &() }

fn main() {
    let y = String::new();
    // This previously compiled, it now errors because `&'static &'y str` is not
    // well formed as 'y does not outlive 'static.
    let _: &'static () = foo(y.as_str());
}

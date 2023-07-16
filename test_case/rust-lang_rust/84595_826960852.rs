rust
trait Foo {
    type Error;
}

struct Grault;
impl Foo for Grault {
    type Error = Bar;
}
enum Bar {
    Corge(String),
}

fn handle(err: <Grault as Foo>::Error) {
    if let <Grault as Foo>::Error::Corge(_) = err {}
}

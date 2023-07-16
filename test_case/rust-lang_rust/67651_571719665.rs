rust
trait From {
    fn from();
}

impl From for () {
    fn from() {}
}

impl From for () {
    fn from() {}
}

fn af() -> impl core::future::Future<Output = ()> {
    async move { From::from() }
}

/*
fn f() -> () {
    From::from()
}
*/


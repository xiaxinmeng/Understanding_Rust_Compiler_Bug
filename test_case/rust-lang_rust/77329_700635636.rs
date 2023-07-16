rust
trait Foo {}
fn bug() -> impl Foo<[(); |_: ()| {}]> {}

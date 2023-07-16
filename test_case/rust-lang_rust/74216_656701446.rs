rust
#[must_use]
trait Foo {
    __rename! { async fn bar() ; }
}

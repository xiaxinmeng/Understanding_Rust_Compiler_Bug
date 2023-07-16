rust
trait Bar {}

fn foo() -> impl Bar {
    unimplemented!()
}

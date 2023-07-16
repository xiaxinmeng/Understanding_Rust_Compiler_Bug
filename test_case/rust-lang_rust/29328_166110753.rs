 rust
trait Foo { type Bar; }
trait Foobar<T> : Foo<Bar=T> {}
struct Quux<'a> {
    foobar: &'a Foobar<String>,
}

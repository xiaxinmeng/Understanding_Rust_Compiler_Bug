rust
trait Foo {
    fn foo_method(&self);
}
trait Bar: Foo {
    fn bar_method(&self);
}

let x: &dyn Bar = /* ... */;
let y: &dyn Foo = x; // compiles

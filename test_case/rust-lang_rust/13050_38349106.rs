 rust
trait Foo<T> {}

let _: ~Foo:Freeze<int>;
let _: ~Foo<int>:Freeze;

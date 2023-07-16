rust
mod foo {
    pub macro m($i:ident) {
        fn $i() {} // if this desugared in to `pub(self)`, i.e. `pub(foo)`, ...
    }
}
foo::m!(f); // then this `f` would be private to `foo`.
fn main() {
    f();
}

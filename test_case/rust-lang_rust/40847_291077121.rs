rust
trait Tr {
    fn f(&self) {}
}

macro m($t:type) {
    impl Tr for $t {
        fn f(&self) {}
    }
}

struct S;
m!(S);
S.f();
//^ `f` to resolves to `Tr::f`; the names in the `impl Tr for S` are irrelevant.

// In general, the particular `impl` is not known when resolving method names,
// so it *cannot* be relevant, e.g.:
fn f<T: Tr>(t: T) {
    t.f(); // we don't know anything about the `impl Tr for T` until monomorphization time
}

rust
trait D {}
trait A<R> {
    type AS;
}
struct As<R>(R);
struct AT;
impl<R: D> A<R> for AT {
    type AS = As<R>;
}
#[repr(packed)]
struct S(<AT as A<S>>::AS);

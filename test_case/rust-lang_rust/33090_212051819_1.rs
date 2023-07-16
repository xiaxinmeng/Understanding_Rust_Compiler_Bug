 rust
struct Zip<A, B> {
    inner: <(A, B) as ZipImpl<A, B>>::Repr,
}

trait ZipImpl<A, B> {
    type Repr;
}

impl<A, B> ZipImpl<A, B> for (A, B) {
    type Repr = (A, B);
}

// ...

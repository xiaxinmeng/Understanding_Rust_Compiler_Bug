rust
trait Foo<const N1: u8, const N2: u8, const N3: u8> {}

struct A<const N1: u8, const N2: u8, const N3: u8>;

type AA = A<1, 2, 3>;

// The compiler actively tells you that the braces on the next line are unnecessary, currently.
type AAA = A<{ 1 }, { 2 }, { 3 }>;

// The braces on the next line are equally unnecessary, but not warned against the way they are
// above for some reason.
impl<const N1: u8, const N2: u8, const N3: u8> Foo<N1, N2, N3> for A<{ N1 }, { N2 }, { N3 }> {}

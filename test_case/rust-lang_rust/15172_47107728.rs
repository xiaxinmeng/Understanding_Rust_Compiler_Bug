 rust
trait Tr<T> {
    fn op(T) -> Self;
}

// these compile as if Self: Tr<U>, even tho only Self: Tr<Self or T>
trait A:    Tr<Self> { fn test<U>(u: U) -> Self { Tr::op(u) } }
trait B<T>: Tr<T>    { fn test<U>(u: U) -> Self { Tr::op(u) } }
// these rightfully fail to compile:
// struct X;
// trait C:    Tr<Self> { fn test   (u: X) -> Self { Tr::op(u) } }
// trait D:    Tr<X>    { fn test<U>(u: U) -> Self { Tr::op(u) } }
// trait E<T>: Tr<T>    { fn test   (u: X) -> Self { Tr::op(u) } }

impl<T> Tr<T> for T {
    fn op(t: T) -> T { t }
}
impl<T> A for T {}

fn main() {
    std::io::println(A::test((&7306634593706211700, 8)));
}

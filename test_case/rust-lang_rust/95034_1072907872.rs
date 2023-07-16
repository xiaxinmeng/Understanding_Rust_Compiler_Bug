rust
trait Tr<A> { type B; type C; }
struct S<A, B, T: Tr<A, B = B>>(<T as Tr<A>>::C);

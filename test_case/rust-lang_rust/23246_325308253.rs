
enum Foo<A, B>
    where A: Bar<B> + Sized
{
    X(A),
    Y,
    Z
}

trait Bar<B> {
   fn bar(&self, &B);
}

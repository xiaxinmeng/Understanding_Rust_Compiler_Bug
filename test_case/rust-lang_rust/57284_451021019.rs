rust
trait A {}
impl<T> A for T {}

struct B;

fn test<'a, 'b>()
where
    &'a B: A,
    &'b B: A,
{
}

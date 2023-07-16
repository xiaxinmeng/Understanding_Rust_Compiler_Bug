 rust
struct Map<A,B,I,F>
    where I : Iterator<Item=A>, F : FnMut(A) -> B
{
    iter: I,
    fn: F
}

impl<A,B,I,F> Iterator<B> for Map<I,F>
    where I : Iterator<Item=A>, F : FnMut(A) -> B
{ ... }

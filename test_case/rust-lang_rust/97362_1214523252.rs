rust
trait AsyncF<'a, Input>: Fn(Input) -> Self::Future {
    type Future: Future<Output = Self::Out>;
    
    type Out;
}

impl<'a, F, Fut, In> AsyncF<'a, In> for F
where
    F: Fn(In) -> Fut,
    Fut: Future,
{
    type Future = Fut;
    type Out = Fut::Output;
}

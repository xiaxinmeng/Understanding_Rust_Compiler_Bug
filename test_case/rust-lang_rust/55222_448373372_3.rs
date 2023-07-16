rust
impl<P0...Pn> FooTrait<P0..Pn> for ()
where
    WF(Bar)
{
    type Out = Bar;
}

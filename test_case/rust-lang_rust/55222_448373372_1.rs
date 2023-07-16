
trait FooTrait<P0...Pn> {
    type Out;
}

impl<P0...Pn> FooTrait<P0..Pn> for () {
    type Out = Bar;
}

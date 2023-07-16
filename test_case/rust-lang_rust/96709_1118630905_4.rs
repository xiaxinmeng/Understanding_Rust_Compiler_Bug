rust
trait CanDoThing {
    type Input<'a>;
    
    fn do_thing(&self, input: Self::Input<'_>);
}

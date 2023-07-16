rust
trait CanDoThing {
    type Input;
    
    fn do_thing(&self, input: Self::Input);
}

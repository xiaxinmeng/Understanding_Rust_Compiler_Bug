 rust
pub trait Processor : Subscriber<Input=<Self as Processor>::Input> {
    type Input;
}

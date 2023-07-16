rust
pub trait Processable<I> {
    type Output;

    fn process(&self, input: I, context: &mut PipelineContext) -> Self::Output;
}

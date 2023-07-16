rust
pub trait Processable {
    type Input;
    type Output;

    fn process(&self, input: Self::Input, context: &mut PipelineContext) -> Self::Output;
}

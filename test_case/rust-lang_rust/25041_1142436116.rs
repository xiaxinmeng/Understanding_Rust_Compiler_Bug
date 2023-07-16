rust

pub struct PipelineContext {
}

impl PipelineContext {
    pub fn teardown(self) -> Box<dyn PipelineProcessor> {
        self.processor
    }
}

pub trait Processable {
    type Input;
    type Output;

    fn process(&self, input: Self::Input, context: &mut PipelineContext) -> Self::Output;
}

pub struct PipelineStep<P, N>
where
    P: Processable,
    N: Processable<Input = P::Output>,
{
    process: P,
    next: N,
}

impl<P, N> PipelineStep<P, N>
where
    P: Processable,
    N: Processable<Input = P::Output>,
{
    pub fn new(process: P, next: N) -> Self {
        Self { process, next }
    }
}

impl<P, N> Processable for PipelineStep<P, N>
where
    P: Processable,
    N: Processable<Input = P::Output>,
{
    type Input = P::Input;
    type Output = N::Output;

    fn process(&self, input: Self::Input, context: &mut PipelineContext) -> Self::Output {
        let output = self.process.process(input, context);
        self.next.process(output, context)
    }
}

pub struct EndStep<I> {
    phantom: PhantomData<I>,
}

impl<I> Default for EndStep<I> {
    fn default() -> Self {
        Self {
            phantom: PhantomData::default(),
        }
    }
}

impl<I> Processable for EndStep<I> {
    type Input = I;
    type Output = I;

    fn process(&self, input: Self::Input, _context: &mut PipelineContext) -> Self::Output {
        input
    }
}

impl<I, O> Processable for &fn(input: I, context: &mut PipelineContext) -> O {
    type Input = I;
    type Output = O;

    fn process(&self, input: Self::Input, context: &mut PipelineContext) -> Self::Output {
        (self)(input, context)
    }
}

impl<I, O> Processable for fn(input: I, context: &mut PipelineContext) -> O {
    type Input = I;
    type Output = O;

    fn process(&self, input: Self::Input, context: &mut PipelineContext) -> Self::Output {
        (self)(input, context)
    }
}

pub struct ClosureProcessable<F, I, O>
where
    F: Fn(I, &mut PipelineContext) -> O,
{
    func: F,
    phantom_i: PhantomData<I>,
}

impl<F, I, O> Processable for ClosureProcessable<F, I, O>
where
    F: Fn(I, &mut PipelineContext) -> O,
{
    type Input = I;
    type Output = O;

    fn process(&self, input: Self::Input, context: &mut PipelineContext) -> Self::Output {
        (self.func)(input, context)
    }
}

#[cfg(test)]
mod tests {
    use crate::io::pipeline::{
        ClosureProcessable, EndStep, PipelineContext, PipelineProcessor, PipelineStep, Processable,
    };
    use std::sync::mpsc;

    pub struct DummyPipelineProcessor;

    impl PipelineProcessor for DummyPipelineProcessor {}

    fn add_one(input: u32, context: &mut PipelineContext) -> u8 {
        input as u8 + 1
    }

    fn add_two(input: u8, context: &mut PipelineContext) -> u32 {
        input as u32 + 2
    }

    #[test]
    fn test() {
        let mut context = PipelineContext {
        };
        let output: u32 = PipelineStep {
            process: add_two as fn(u8, &mut PipelineContext) -> u32,
            next: EndStep::default(),
        }
        .process(5u8, &mut context);

        assert_eq!(output, 7);

        let output = PipelineStep {
            process: add_one as fn(u32, &mut PipelineContext) -> u8,
            next: PipelineStep {
                process: add_two as fn(u8, &mut PipelineContext) -> u32,
                next: EndStep::default(),
            },
        }
        .process(5u32, &mut context);

        assert_eq!(output, 8);

        let output: u32 = PipelineStep {
            process: ClosureProcessable {
                func: |input: u8, context| -> u32 {
                    return input as u32 + 2;
                },
                phantom_i: Default::default(),
            },
            next: EndStep::default(),
        }
        .process(5u8, &mut context);

        assert_eq!(output, 7);
    }
}

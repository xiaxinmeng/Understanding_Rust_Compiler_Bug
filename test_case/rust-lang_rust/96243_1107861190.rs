rust
pub trait Womble<R>
{
    fn is_womble(&self, a: &R) -> bool;
}

struct WrapWomble<S> 
{
    womble: S
}

impl<'s, R> Womble<R> for WrapWomble<Box<dyn Womble<R> + 's>>
{
    fn is_womble(&self, a: &R) -> bool {
        self.womble.is_womble(a)
    }
}

fn stack_wombles<'s, R>(a: Box<dyn Womble<R> + 's>) -> Box<dyn Womble<R> + 's> {
    Box::new(WrapWomble { womble: a })
}

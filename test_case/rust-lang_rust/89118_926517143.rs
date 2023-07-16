rust
trait BufferMut {}
struct Ctx<D>(D);

trait BufferUdpStateContext<B> {}
impl<B: BufferMut, C> BufferUdpStateContext<B> for C {}

trait StackContext
where
    Ctx<()>: for<'a> BufferUdpStateContext<&'a ()>,
{
    type Dispatcher;
}

trait TimerContext {
    type Handler;
}
impl<C> TimerContext for C
where
    C: StackContext,
{
    type Handler = Ctx<C::Dispatcher>;
}

struct EthernetWorker<C>(C)
where
    Ctx<()>: for<'a> BufferUdpStateContext<&'a ()>;
impl<C> EthernetWorker<C> {}

fn main() {}

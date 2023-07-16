 rust
use std::marker::PhantomData;
use std::sync::atomic::AtomicUsize;
struct Receipt<A: Async> {
    core: (),
    count: u64,
    marker: PhantomData<A>,
}
struct Complete<T, E> {
    core: Option<Core<T, E>>,
}
impl <T: Send, E: Send> Async for Complete<T, E> {type
    Cancel
    =
    Receipt<Complete<T, E>>;
}

impl <T: Send, E: Send> Cancel<Complete<T, E>> for Receipt<Complete<T, E>> {
    fn cancel(self) -> Option<Complete<T, E>> { }
}

struct Core<T, E> {
    refs: AtomicUsize,
    state: AtomicState,
    consumer_wait: Option<Callback<T, E>>,
    producer_wait: Option<Callback<T, E>>,
    val: AsyncResult<T, E>, //
}
struct AtomicState;
type Callback<T, E> = Box<BoxedReceive<Core<T, E>>>;
trait Async {type Cancel: Cancel<Self>; }
trait Cancel<A> {
    fn cancel(self) -> Option<A>;
}
type AsyncResult<T, E> = Result<T, AsyncError<E>>;
enum AsyncError<E> { ExecutionError(E), }
trait BoxedReceive<T> {
    fn receive_boxed(Box<Self>, T);
}

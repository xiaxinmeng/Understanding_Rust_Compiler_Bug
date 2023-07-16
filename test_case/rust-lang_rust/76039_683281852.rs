rust

struct AsyncFnReturn;

type FutureType = AsyncFnReturn;

fn my_future() -> AsyncFnReturn { AsyncFnReturn }
fn consume(_: FutureType) { }
fn create() -> FutureType { my_future() }

fn main() {
    consume(create());
    consume(my_future());
}

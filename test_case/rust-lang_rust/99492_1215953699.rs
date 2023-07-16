rs
use std::marker::PhantomData;

#[rustfmt::skip]
pub struct Struct<I, T, F, B>(
    PhantomData<fn() -> (I, T, F, B)>,
    PhantomData<<Self as It>::Item>,
)
where
    I: It<Item = T>, // xxx
    F: FnMut(T) -> B, // yyy
;

impl<I, T, F, B> Struct<I, T, F, B>
where
    I: It<Item = T>, // xxx
    F: FnMut(T) -> B, // yyy
{
    fn new(_: F) -> Self {
        Self(PhantomData, PhantomData)
    }
}
impl<B, I, T, F> It for Struct<I, T, F, B>
where
    I: It<Item = T>, // xxx
    F: FnMut(T) -> B, // yyy
{
    type Item = B;
}

pub trait It {
    type Item;
}

async fn execution_process() {
    let _x = Struct::<Empty<&u64>, &u64, _, u64>::new(|seq: &u64| *seq);
    async {}.await;
}

fn spawn_execute_process() {
    spawn(execution_process());
}

fn spawn<T>(_: T)
where
    T: Send, // zzz
{
}

struct FnReturning<T>(fn() -> T);

pub struct Empty<T>(PhantomData<FnReturning<T>>);

impl<T> It for Empty<T> {
    type Item = T;
}

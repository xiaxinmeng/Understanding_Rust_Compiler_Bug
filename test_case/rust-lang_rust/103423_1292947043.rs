rust
#![feature(generators)]
#![feature(trait_alias)]
#![feature(type_alias_impl_trait)]

use std::{borrow::Cow, convert::Infallible};

use futures::{executor::block_on, pin_mut, Future, Stream, StreamExt};
use futures_async_stream::stream;

pub trait AsyncIterator: 'static {
    type NextFuture<'a>: Future<Output = Result<(), Infallible>> + 'a
    where
        Self: 'a;

    fn next(&mut self) -> Self::NextFuture<'_>;
}

pub struct GoodIterator;

impl AsyncIterator for GoodIterator {
    type NextFuture<'a> = impl Future<Output = Result<(), Infallible>> + 'a;
    fn next(&mut self) -> Self::NextFuture<'_> {
        async move { futures::future::pending().await }
    }
}

pub struct BadIterator<const T: usize, I: AsyncIterator> {
    children: Vec<I>,
}

impl<const T: usize, I: AsyncIterator> BadIterator<T, I> {
    pub fn new(children: Vec<I>) -> Self {
        Self { children }
    }
}

impl<const T: usize, I: AsyncIterator> AsyncIterator for BadIterator<T, I> {
    type NextFuture<'a> = impl Future<Output = Result<(), Infallible>> + 'a;
    fn next(&mut self) -> Self::NextFuture<'_> {
        async move {
            for i in self.children.iter_mut() {
                i.next().await?;
            }
            Ok(())
        }
    }
}

pub enum IteratorUnion<I1: AsyncIterator, I2: AsyncIterator, I3: AsyncIterator, I4: AsyncIterator> {
    I1(I1),
    I2(I2),
    I3(I3),
    I4(I4),
}

impl<I1: AsyncIterator, I2: AsyncIterator, I3: AsyncIterator, I4: AsyncIterator> AsyncIterator
    for IteratorUnion<I1, I2, I3, I4>
{
    type NextFuture<'a> = impl Future<Output = Result<(), Infallible>> + 'a;
    fn next(&mut self) -> Self::NextFuture<'_> {
        async move {
            match self {
                Self::I1(x) => x.next().await,
                Self::I2(x) => x.next().await,
                Self::I3(x) => x.next().await,
                Self::I4(x) => x.next().await,
            }
        }
    }
}

fn create_iterator<const D: usize>() -> impl AsyncIterator {
    let iter1 = IteratorUnion::I1(BadIterator::<D, _>::new(vec![GoodIterator, GoodIterator]));
    let iter2 = IteratorUnion::I2(BadIterator::<D, _>::new(vec![GoodIterator, GoodIterator]));
    let iter3 = IteratorUnion::I3(BadIterator::<D, _>::new(vec![GoodIterator, GoodIterator]));
    let iter4 = IteratorUnion::I4(BadIterator::<D, _>::new(vec![GoodIterator, GoodIterator]));
    BadIterator::<233, _>::new(vec![iter1, iter2, iter3, iter4])
}

fn create_iterator_2<const D: usize>() -> impl AsyncIterator {
    let iter1 = IteratorUnion::I1(create_iterator::<1>());
    let iter2 = IteratorUnion::I2(create_iterator::<2>());
    let iter3 = IteratorUnion::I3(create_iterator::<3>());
    let iter4 = IteratorUnion::I4(create_iterator::<4>());
    BadIterator::<D, _>::new(vec![iter1, iter2, iter3, iter4])
}

fn create_iterator_3<const D: usize>() -> impl AsyncIterator {
    let iter1 = IteratorUnion::I1(create_iterator_2::<1>());
    let iter2 = IteratorUnion::I2(create_iterator_2::<2>());
    let iter3 = IteratorUnion::I3(create_iterator_2::<3>());
    let iter4 = IteratorUnion::I4(create_iterator_2::<4>());
    BadIterator::<D, _>::new(vec![iter1, iter2, iter3, iter4])
}

fn create_iterator_4<const D: usize>() -> impl AsyncIterator {
    let iter1 = IteratorUnion::I1(create_iterator_3::<1>());
    let iter2 = IteratorUnion::I2(create_iterator_3::<2>());
    let iter3 = IteratorUnion::I3(create_iterator_3::<3>());
    let iter4 = IteratorUnion::I4(create_iterator_3::<4>());
    BadIterator::<D, _>::new(vec![iter1, iter2, iter3, iter4])
}

fn create_iterator_5<const D: usize>() -> impl AsyncIterator {
    let iter1 = IteratorUnion::I1(create_iterator_4::<1>());
    let iter2 = IteratorUnion::I2(create_iterator_4::<2>());
    let iter3 = IteratorUnion::I3(create_iterator_4::<3>());
    let iter4 = IteratorUnion::I4(create_iterator_4::<4>());
    BadIterator::<D, _>::new(vec![iter1, iter2, iter3, iter4])
}

type MyFavoriteIterator = impl AsyncIterator;

fn create_iterator_6(x: usize) -> MyFavoriteIterator {
    create_iterator_4::<23333>()
}

#[stream(item = (Cow<'a, ()>, Cow<'a, ()>))]
async fn next<'a>(mut i: impl AsyncIterator) {
    loop {
        match i.next().await {
            Ok(_) => {}
            Err(_) => {}
        }
        yield (Cow::Owned(()), Cow::Owned(()));
    }
}

#[stream(item = (Cow<'a, ()>, Cow<'a, ()>))]
async fn next2<'a, S1, S2>( s1: S1,  s2: S2)
where
    S1: Stream<Item = (Cow<'a, ()>, Cow<'a, ()>)> + 'a,
    S2: Stream<Item = (Cow<'a, ()>, Cow<'a, ()>)>+ 'a,
{
    pin_mut!(s1);
    pin_mut!(s2);
    s1.next().await.unwrap();
    s2.next().await.unwrap();
}

macro_rules! repeat {
    ($x:expr) => ();
    ($x:expr, $($y:expr),+) => {
        block_on(async {
            let iter1 = create_iterator_6(1);
            let iter1 = next(iter1);
            let iter2 = create_iterator_6(1);
            let iter2 = next(iter2);
            let iter = next2(iter1, iter2);
            pin_mut!(iter);
            while let Some(_) = iter.next().await {}
        });
        repeat!($($y),+);
    };
}
fn main() {
    repeat!(
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1
    );
}

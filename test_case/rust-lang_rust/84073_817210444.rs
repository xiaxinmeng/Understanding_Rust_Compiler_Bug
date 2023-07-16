rust
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

mod race {
    use core::future::Future;
    use core::marker::PhantomData;
    use core::pin::Pin;
    use core::task::{Context, Poll};

    pub struct RaceFuture<T, F, S, G, U>
        where F: StatefulFuture<Option<T>, S>,
              G: Future<Output = Option<U>>,
    {
        inner: F,
        future: G,
        event: fn(&mut S, U) -> T,
    }

    impl<T, F, S, G, U> StatefulFuture<Option<T>, S> for RaceFuture<T, F, S, G, U>
        where F: StatefulFuture<Option<T>, S>,
              G: Future<Output = Option<U>>,
    {
        fn poll(&mut self, cx: &mut Context<'_>, state: &mut S) -> Poll<Option<T>> {
            Poll::Pending // FIXME
        }
    }

    pub trait StatefulFuture<T, S> {
        fn poll(&mut self, cx: &mut Context<'_>, state: &mut S) -> Poll<T>;
    }

    pub struct Never<T, S>(PhantomData<(T, S)>);

    impl<T, S> StatefulFuture<T, S> for Never<T, S> {
        fn poll(&mut self, cx: &mut Context<'_>, state: &mut S) -> Poll<T> {
            Poll::Pending
        }
    }

    /// Race builder.  Add tasks by calling `when()`.
    pub struct RaceBuilder<T, F, S>
        where F: StatefulFuture<Option<T>, S>
    {
        future: F,
        _phantom: PhantomData<(S, T)>,
    }

    impl<T, F, S> RaceBuilder<T, F, S>
        where F: StatefulFuture<Option<T>, S>
    {
        pub fn when<U, G>(self, future: G, event: fn(&mut S, U) -> T)
                -> RaceBuilder<T, RaceFuture<T, F, S, G, U>, S>
            where G: Future<Output = Option<U>>,
        {
            RaceBuilder {
                future: RaceFuture {
                    inner: self.future, future, event
                },
                _phantom: PhantomData,
            }
        }
    }

    /// A future that returns a closure for the first completed future.
    #[derive(Debug)]
    pub struct Race<'a, T, S, R, F>
        where R: Fn(RaceBuilder<T, Never<T, S>, S>, &mut S) -> RaceBuilder<T, F, S>,
              F: StatefulFuture<Option<T>, S>,
    {
        state: &'a mut S,
        race: R,
        _phantom: PhantomData<(T, F)>,
    }

    impl<'a, T, S, R, F> Race<'a, T, S, R, F>
        where R: Fn(RaceBuilder<T, Never<T, S>, S>, &mut S) -> RaceBuilder<T, F, S>,
              F: StatefulFuture<Option<T>, S>,
    {
        pub fn new(state: &'a mut S, race: R) -> Self {
            Self { state, race, _phantom: PhantomData }
        }
    }

    impl<T, S, R, F> Future for Race<'_, T, S, R, F>
        where R: Fn(RaceBuilder<T, Never<T, S>, S>, &mut S) -> RaceBuilder<T, F, S>,
              F: StatefulFuture<Option<T>, S>,
    {
        type Output = T;
        
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
            Poll::Pending // FIXME
        }
    }
}

/// Shared state between tasks on the thread.
struct State {
    counter: usize,
    one: Never,
}

impl State {
    fn one(&mut self, _: ()) -> Option<()> {
        todo!()
    }
}

struct Never();

impl Never {
    fn new() -> Self {}
}

impl Future for Never {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        Poll::Pending
    }
}

async fn run() {
    let mut state = State {
        counter: 0,
        one: Never::new(),
    };

    loop {
        if let Some(x) = race::Race::new(&mut state, |race, state| {
            race.when(&mut state.one, State::one)
        }).await {
            break x;
        }
    }
}

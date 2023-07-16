rust
impl<T, E> Bubble for YeetSuccess<Result<T, E>> {
    type Continue = E;
    type Holder = YeetSuccess<Result<T, !>>;
    fn continue_with(e: E) -> Self {
        Self(Err(e))
    }
    fn branch(self) -> ControlFlow<YeetSuccess<Result<T, !>>, E> {
        match self.0 {
            Ok(t) => ControlFlow::Break(YeetSuccess(Ok(t))),
            Err(e) => ControlFlow::Continue(e),
        }
    }
}

impl<T> Bubble for YeetSuccess<Option<T>> {
    type Continue = ();
    type Holder = YeetSuccess<Option<T>>;
    fn continue_with(_: ()) -> Self {
        Self(None)
    }
    fn branch(self) -> ControlFlow<YeetSuccess<Option<T>>, ()> {
        match self.0 {
            Some(t) => ControlFlow::Break(YeetSuccess(Some(t))),
            None => ControlFlow::Continue(()),
        }
    }
}

impl<T, E> BreakHolder<E> for YeetSuccess<Result<T, !>> {
    type Output = YeetSuccess<Result<T, E>>;
}

impl<T> BreakHolder<()> for YeetSuccess<Option<T>> {
    type Output = YeetSuccess<Option<T>>;
}

impl<T, E> Try<YeetSuccess<Result<T, !>>> for Result<T, E> {
    fn from_holder(holder: YeetSuccess<Result<T, !>>) -> Self {
        match holder.0 {
            Ok(t) => Ok(t),
            Err(e) => match e {},
        }
    }
}

impl<T> Try<YeetSuccess<Option<T>>> for Option<T> {
    fn from_holder(holder: YeetSuccess<Option<T>>) -> Self {
        holder.0
    }
}

impl<H, R> Try<YeetSuccess<H>> for YeetSuccess<R>
where
    YeetSuccess<R>: Bubble,
    R: Try<YeetSuccess<H>>,
{
    fn from_holder(holder: YeetSuccess<H>) -> Self {
        YeetSuccess(R::from_holder(holder))
    }
}

rust
impl<V, C: catch::Catchable> std::ops::Try for GuardedResult<V, C, join::Joined> {
    type Output = V;

    type Residual = GuardedResult<V, catch::Uncaught, join::Unjoined>;

    fn from_output(output: Self::Output) -> Self {
        //
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        todo!()
    }
}

impl<V> std::ops::Residual<V> for GuardedResult<V, catch::Uncaught, join::Unjoined> {
    fn from_residual(r: GuardedResult<Infallible, catch::Uncaught, join::Unjoined>) -> GuardedResult<V, catch::Uncaught, join::Unjoined> {
        GuardedResult { _c: PhantomData::default(), _j: PhantomData::default(), ..r }
    }
}

/// the struct in question
pub struct GuardedResult<V, C: catch::Catchable, J: join::Joinable> {
    value: Option<V>,
    root_error: Option<ParseResultError>,
    cascading_errors: ErrorSet,

    solution: SolutionClass,

    /// If this error has been caught and is planned to be locally handled, this is true
    caught: bool,

    _c: PhantomData<C>,
    _j: PhantomData<J>,
}

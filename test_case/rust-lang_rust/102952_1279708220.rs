rust
// wasmi/src/engine/executor.rs
pub trait Execute: Sized {
    fn execute_dummy(self) -> Result<CallOutcome, TrapCode>;
}

impl<'ctx, 'engine, 'func> Execute for Executor<'ctx, 'engine, 'func, ()> {
    fn execute_dummy(self) -> Result<CallOutcome, TrapCode> {
        self.execute()
    }
}

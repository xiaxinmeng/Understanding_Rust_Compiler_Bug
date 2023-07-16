rust
impl Condvar {
    fn wait_until<'a, T, F>(
        &self,
        guard: MutexGuard<'a, T>,
        condition: F,
    ) -> LockResult<MutexGuard<'a, T>>
    where
        F: FnMut(&mut T) -> bool;

    fn wait_timeout_until<'a, T, F>(
        &self,
        guard: MutexGuard<'a, T>,
        dur: Duration,
        condition: F,
    ) -> LockResult<(MutexGuard<'a, T>, WaitTimeoutResult)>
    where
        F: FnMut(&mut T) -> bool;
}

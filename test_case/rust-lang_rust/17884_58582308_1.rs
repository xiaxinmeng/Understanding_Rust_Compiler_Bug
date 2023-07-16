
impl<T, U> Clone for (T, U)
    where T:Clone, U:Clone, (T,U):!Copy { ... }

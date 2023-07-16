rust
trait UnwrapExt {
    type Inner;
    fn guaranteed_unwrap(self) -> Self::Inner;
}

impl<T, E> UnwrapExt for Result<T, E> {
    type Inner = T;
    fn guaranteed_unwrap(self) -> T {
        extern "C" {
            fn failed_to_optimize_unwrap_branch_check();
        }
        // This causes a linker error if the unwrap isn't optimized away, should probably 
        // only be used if your crate does not have any dependencies.
        self.unwrap_or_else(|_| failed_to_optimize_unwrap_branch_check())
    }
}

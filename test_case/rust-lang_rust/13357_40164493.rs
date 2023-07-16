 rust
use sync::mutex::{MUTEX_INIT, StaticMutex};

pub struct TryOnce {
    mutex: StaticMutex,
    is_done: bool,
}

pub static TRY_ONCE_INIT: TryOnce = TryOnce {
    mutex: MUTEX_INIT,
    is_done: false,
};

impl TryOnce {
    /// Attempt to call `f`.
    ///
    /// # Arguments
    ///
    /// - `f`: A function that will only be called if `TryOnce::try` has not
    ///   previously been called successfully.
    ///   - To indicate that the function call was successful an `OK(_)` value
    ///     should be returned. This will prevent  all subsequent subsequent
    ///     calls to `TryOnce::try` from executing `f` and `Some(OK(_))` to be
    ///     returned.
    ///   - To indicate that the function call failed an `Err(_)` value should
    ///     be returned. This will allow `TryOnce::try` to be called again, and
    ///     will cause `TryOnce::try` to return `Some(Err(_))`.
    ///
    /// # Returns
    ///
    /// - `Some(_)` if `TryOnce::try` has never been successfully called.
    /// - `None` if `TryOnce::try` has previously returned `Some(Ok(_))`.
    ///
    /// # Example
    ///
    /// ~~~rust
    /// # mod ffi {
    /// #     pub static FALSE: int = 0;
    /// #     pub static TRUE:  int = 1;
    /// #     pub fn init() -> int { TRUE }
    /// # }
    ///
    /// enum InitError {
    ///     InternalInitError,
    ///     AlreadyInitialized,
    /// }
    ///
    /// fn init() -> Result<~str, InitError> {
    ///     static mut INIT: TryOnce = TRY_ONCE_INIT;
    ///     unsafe {
    ///         INIT.try(&|| {
    ///             if ffi::init() == ffi::TRUE {
    ///                 Ok(~"hi!")
    ///             } else {
    ///                 Err(InternalInitError)
    ///             }
    ///         })
    ///     }.unwrap_or(Err(AlreadyInitialized))
    /// }
    /// ~~~
    pub fn try<T, E>(&mut self, f: &|| -> Result<T, E>) -> Option<Result<T, E>> {
        let guard = self.mutex.lock();
        let result = if self.is_done {
            None
        } else {
            let result = (*f)();
            self.is_done = result.is_ok();
            Some(result)
        };
        drop(guard);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::{TRY_ONCE_INIT, TryOnce};

    #[deriving(Eq, Show)]
    enum InitError {
        InternalInitError,
        AlreadyInitialized,
    }

    #[test]
    fn test() {
        static mut INIT: TryOnce = TRY_ONCE_INIT;

        let err: &|| -> Result<~str, InitError> = &|| Err(InternalInitError);
        let ok:  &|| -> Result<~str, InitError> = &|| Ok(~"pickles");

        assert_eq!(unsafe { INIT.try(err) }, Some(Err(InternalInitError)));
        assert_eq!(unsafe { INIT.try(ok)  }, Some(Ok(~"pickles")));
        assert_eq!(unsafe { INIT.try(err) }, None);
        assert_eq!(unsafe { INIT.try(ok)  }, None);
    }
}

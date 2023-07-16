rust
mod fake_thread_local {
    pub struct LocalKey<T: 'static> {
        pub get_value: fn() -> Option<&'static T>,
    }
    impl<T: 'static> LocalKey<T> {
        pub fn with<F, R>(&'static self, f: F) -> R
        where
            F: FnOnce(&T) -> R,
        {
            self.try_with(f).expect("")
        }

        #[inline]
        pub fn try_with<F, R>(&'static self, f: F) -> Option<R>
        where
            F: FnOnce(&T) -> R,
        {
            let fake_thread_local = (self.get_value)()?;
            Some(f(fake_thread_local))
        }
    }
}

fn get_value() -> Option<&'static ()> {
    Some(&())
}

const THREAD_LOCAL_GLOBAL: fake_thread_local::LocalKey<()> = fake_thread_local::LocalKey { get_value };

#[inline(never)]
fn set_state_func(_: &()) {
}


/// # Examples
///
/// 
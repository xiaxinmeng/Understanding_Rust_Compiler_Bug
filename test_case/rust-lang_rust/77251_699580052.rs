rust
pub mod thelibrary {
    use lazy_static::lazy_static;
    use std::ptr;
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::sync::Mutex;

    pub struct Options {
        pub include_prefix: &'static str,
    }

    lazy_static! {
        static ref INCLUDE_PREFIX: Mutex<&'static str> = Mutex::new(env!("CARGO_CRATE_NAME"));
    }

    impl Options {
        fn current() -> Self {
            Options {
                include_prefix: *INCLUDE_PREFIX.lock().unwrap(),
            }
        }
    }

    use private::Cfg;
    pub const CFG: Cfg = Cfg {
        options: AtomicPtr::new(ptr::null_mut()),
    };
    mod private {
        use super::*;
        pub struct Cfg {
            pub(super) options: AtomicPtr<Options>,
        }
    }

    impl std::ops::Deref for Cfg {
        type Target = Options;
        fn deref(&self) -> &Self::Target {
            let options = Box::into_raw(Box::new(Options::current()));
            let prev = self
                .options
                .compare_and_swap(ptr::null_mut(), options, Ordering::Relaxed);
            if !prev.is_null() {
                // compare_and_swap did nothing.
                let _ = unsafe { Box::from_raw(options) };
                panic!();
            }
            unsafe { &*options }
        }
    }

    impl std::ops::DerefMut for Cfg {
        fn deref_mut(&mut self) -> &mut Self::Target {
            let options = self.options.get_mut();
            if !options.is_null() {
                panic!();
            }
            *options = Box::into_raw(Box::new(Options::current()));
            unsafe { &mut **options }
        }
    }

    impl Drop for Cfg {
        fn drop(&mut self) {
            let options = *self.options.get_mut();
            if let Some(options) = unsafe { options.as_mut() } {
                *INCLUDE_PREFIX.lock().unwrap() = options.include_prefix;
                let _ = unsafe { Box::from_raw(options) };
            }
        }
    }
}

use thelibrary::CFG;

fn main() {
    CFG.include_prefix = "path/to";

    println!("{:?}", CFG.include_prefix);
}

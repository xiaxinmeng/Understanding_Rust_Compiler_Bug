rust
// core crate:

#[doc(hidden)]
#[unstable(...)] // perma-unstable interface for libstd
pub trait BacktraceImpl: Debug + Display {
    // can add more functions here since it's a perma-unstable trait
}

#[stable(...)]
pub struct Backtrace(Box<dyn BacktraceImpl>);

impl Backtrace {
    #[unstable(...)]
    #[doc(hidden)]
    pub fn new(v: Box<dyn BacktraceImpl>) {
        Self(v)
    }
    // can add more functions here since it's not a trait
}

impl Display for Backtrace { ... }
impl Debug for Backtrace { ... }

// std crate:

struct StdBacktrace {
   ...
}

impl BacktraceImpl for StdBacktrace { ... }

// use whatever API we decide on for making a backtrace:
#[stable(...)]
pub fn make_a_backtrace() -> Backtrace {
    Backtrace::new(Box::new(StdBacktrace { ... }))
}

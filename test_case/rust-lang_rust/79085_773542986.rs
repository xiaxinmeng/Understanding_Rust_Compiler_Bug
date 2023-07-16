rust
/// Global implementation of backtrace functinality. Called to create
/// `RawBacktrace` trait objects.
extern {
    #[linkage = "extern_weak"]
    #[link_name = "__rust_backtrace_impl"]
    static BACKTRACE_IMPL: *mut &'static dyn BacktraceImpl;
}

// perma(?)-unstable
#[unstable(feature = "core_backtrace", issue = "74465")]
pub trait BacktraceImpl: 'static {
    fn capture(&self) -> *mut dyn RawBacktrace;
    fn enabled(&self) -> bool;
}

// perma(?)-unstable
#[unstable(feature = "core_backtrace", issue = "74465")]
pub trait RawBacktrace: fmt::Debug + fmt::Display + 'static {
    unsafe fn drop_and_free(self: *mut Self);
}

#[unstable(feature = "core_backtrace", issue = "74465")]
pub struct Backtrace {
    inner: *mut dyn RawBacktrace,
}

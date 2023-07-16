Rust
#[doc(hidden)]
#[unstable(feature="libstd_implementation_detail")]
pub trait ErrorOrStrMapper {
    type Output;
    fn from_error<E: Error>(self, e: E) -> Self::Output;
    fn from_str(self, s: &str) -> Self::Output;
}

#[doc(hidden)]
#[unstable(feature="libstd_implementation_detail")]
trait ErrorOrStr {
    fn convert<F: ErrorOrStrMapper>(self, f: F) -> F::Output;
}

impl<E: Error> ErrorOrStr for E {
    fn to_error<F: ErrorOrStrMapper>(self, f: F) -> F::Output {
        f.from_error(self)
    }
}

// ok because libcore knows that `! &'a str : Error`
impl<'a> ErrorOrStr for &'a str {
    fn to_error<F: ErrorOrStrMapper>(self, f: F) -> F::Output {
        f.from_str(self)
    }
}

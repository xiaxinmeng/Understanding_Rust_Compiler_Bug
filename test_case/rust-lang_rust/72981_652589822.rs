
#![feature(backtrace)]

use std::fmt::{Debug, Display};

/// Backtrace trait in libcore
trait Backtrace: Display + Debug {}

/// Error trait in libcore too?
trait Error {
    fn backtrace(&self) -> Option<&dyn Backtrace>;
}

/// StdBacktrace type in libstd
struct StdBacktrace(std::backtrace::Backtrace);

impl Debug for StdBacktrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         std::fmt::Debug::fmt(&self.0, f)
    }
}

impl Display for StdBacktrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         std::fmt::Display::fmt(&self.0, f)
    }
}

impl Backtrace for StdBacktrace {}


pub fn blah(_: extern "C" fn(str)) {}
pub fn blah(_: extern "C" fn() -> str) {}
pub extern "C" fn blah(_: str) {} // requires #![feature(unsized_locals)]

 java
// Test rustc correctly handle input file with relative path
// and outputs properly named library.

// file: foo.rs
// compile-cmd:--lib ./foo.rs
fn main() { }

// run:ls libfoo-8e702f2a2bfac15b-0.0.so

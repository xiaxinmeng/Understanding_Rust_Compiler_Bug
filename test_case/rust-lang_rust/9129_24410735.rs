 rust
#[cfg(labelled_again)]
pub fn main() { 'foo: loop { loop 'foo; } }

#[cfg(labelled_break)]
pub fn main() { 'foo: loop { break 'foo; } }

#[cfg(labelled_for)]
pub fn main() { 'foo: for i in range(1,10) { } }

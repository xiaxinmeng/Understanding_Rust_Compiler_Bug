rust
macro foo() { 2 + 3 }
fn main() { 1 * foo!() }

=> expand and reparse the whole crate =>

macro foo() { 2 + 3 }
fn main() { 1 * 2 + 3 } // Oh, wait

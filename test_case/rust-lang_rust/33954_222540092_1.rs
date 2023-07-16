 rust
#[cfg(test)]
fn exit() { panic!("exit") }

#[cfg(not(test))]
use std::process::exit;

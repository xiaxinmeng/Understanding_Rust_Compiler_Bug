 rust
foo.wait_timeout_with(lock, duration, |state| Ok(try!(state).condition()));

 rust
#[cfg_attr(feature="nightly", path="nightly_stuff.rs")]
#[cfg_attr(not(feature="nightly"), path="stable_stuff.rs")]
mod stuff;

rust
#[cfg(target_os = "redox")]
fn do_stuff() { /* Redox stuff */ }

#[cfg(unix)]
fn do_stuff() { /* Unix stuff */ }

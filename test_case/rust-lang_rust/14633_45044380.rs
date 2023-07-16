 rust
// liblibc/lib.rs
#[doc(hidden)]
pub fn issue_14344_workaround() {}

// libstd/lib.rs
#[doc(hidden)]
pub fn issue_14344_workaround() {
    libc::issue_14344_workaround();
}

plain
   Compiling rustc-demangle v0.1.21
error: unexpected `cfg` condition name
   --> library/alloc/src/lib.rs:206:11
    |
206 | #[cfg(not(no_rc))]
    |
    |
    = note: `-D unexpected-cfgs` implied by `-D warnings`
error: unexpected `cfg` condition name
   --> library/alloc/src/lib.rs:211:15
    |
    |
211 | #[cfg(all(not(no_rc), not(no_sync), target_has_atomic = "ptr"))]

error: unexpected `cfg` condition name
   --> library/alloc/src/lib.rs:211:27
    |
    |
211 | #[cfg(all(not(no_rc), not(no_sync), target_has_atomic = "ptr"))]

error: unexpected `cfg` condition name
   --> library/alloc/src/lib.rs:213:44
    |
    |
213 | #[cfg(all(not(no_global_oom_handling), not(no_rc), not(no_sync), target_has_atomic = "ptr"))]

error: unexpected `cfg` condition name
   --> library/alloc/src/lib.rs:213:56
    |
    |
213 | #[cfg(all(not(no_global_oom_handling), not(no_rc), not(no_sync), target_has_atomic = "ptr"))]

error: could not compile `alloc` due to 5 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed

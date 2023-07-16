rust
[01:12:37]    Compiling git2 v0.6.7
[01:12:39] error[E0308]: mismatched types
[01:12:39]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/git2-0.6.7/src/blob.rs:117:53
[01:12:39]     |
[01:12:39] 117 |             let res = ((*self.raw).write)(self.raw, buf.as_ptr() as *const i8, buf.len());
[01:12:39]     |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^ expected u8, found i8
[01:12:39]     |
[01:12:39]     = note: expected type `*const u8`
[01:12:39]                found type `*const i8`
[01:12:39] 
[01:12:40] error: aborting due to previous error
[01:12:40] 
[01:12:40] error: Could not compile `git2`.

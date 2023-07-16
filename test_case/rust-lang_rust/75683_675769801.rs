plain
[RUSTC-TIMING] addr2line test:false 0.406
[RUSTC-TIMING] core test:false 21.702
[RUSTC-TIMING] gimli test:false 5.157
[RUSTC-TIMING] object test:false 10.395
error: borrow of packed field is unsafe and requires unsafe function or block (error E0133)
    |
    |
913 |         self.inner.s_addr.hash(s)
    |
    |
    = note: `-D safe-packed-borrows` implied by `-D warnings`
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #46043 <https://github.com/rust-lang/rust/issues/46043>
    = note: fields of packed structs might be misaligned: dereferencing a misaligned pointer or even just creating a misaligned reference is undefined behavior
error: aborting due to previous error

[RUSTC-TIMING] std test:false 3.351
error: could not compile `std`.
---
== clock drift check ==
  local time: Tue Aug 18 23:33:21 UTC 2020
  network time: Tue, 18 Aug 2020 23:33:21 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (2808) (node)
Terminate orphan process: pid (2836) (python)

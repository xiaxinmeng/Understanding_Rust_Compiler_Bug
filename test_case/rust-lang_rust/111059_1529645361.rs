plain
    Checking addr2line v0.19.0
error[E0308]: mismatched types
   --> library/std/src/sys/windows/os_str.rs:170:33
    |
170 |         (prefix, Slice { inner: suffix })
    |                                 ^^^^^^ expected `Wtf8`, found `&Wtf8`
error[E0308]: mismatched types
   --> library/std/src/sys/windows/os_str.rs:170:18
    |
    |
170 |         (prefix, Slice { inner: suffix })
    |                  |
    |                  |
    |                  expected `&Slice`, found `Slice`
    |                  help: consider borrowing here: `&Slice { inner: suffix }`
error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
   --> library/std/src/sys/windows/os_str.rs:170:18
    |
    |
170 |         (prefix, Slice { inner: suffix })
    |
    = help: within `Slice`, the trait `core::marker::Sized` is not implemented for `[u8]`
note: required because it appears within the type `Slice`
   --> library/std/src/sys/windows/os_str.rs:49:12
---

error[E0308]: mismatched types
   --> library/std/src/sys/windows/os_str.rs:243:9
    |
242 |     pub fn strip_prefix<'a, P: Pattern<'a>>(&'a self, prefix: P) -> Option<&'a Slice> {
    |                                                                     ----------------- expected `core::option::Option<&'a Slice>` because of return type
243 |         self.inner.strip_prefix(prefix)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Option<&Slice>`, found `Option<&Wtf8>`
    |
    = note: expected enum `core::option::Option<&'a Slice>`
               found enum `core::option::Option<&Wtf8>`
error[E0308]: mismatched types
   --> library/std/src/sys/windows/os_str.rs:248:9
    |
    |
247 |     pub fn strip_prefix_str(&self, prefix: &str) -> Option<&Slice> {
    |                                                     -------------- expected `core::option::Option<&Slice>` because of return type
248 |         self.inner.strip_prefix_str(prefix)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Option<&Slice>`, found `Option<&Wtf8>`
    |
    = note: expected enum `core::option::Option<&Slice>`
               found enum `core::option::Option<&Wtf8>`
error[E0308]: mismatched types
   --> library/std/src/sys/windows/os_str.rs:253:9
    |
    |
252 |     pub fn split_once<'a, P: Pattern<'a>>(&'a self, delimiter: P) -> Option<(&'a str, &'a Slice)> {
    |                                                                      ---------------------------- expected `core::option::Option<(&'a str, &'a Slice)>` because of return type
253 |         self.inner.split_once(delimiter)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Option<(&str, &Slice)>`, found `Option<(&str, &Wtf8)>`
    |
    = note: expected enum `core::option::Option<(&'a str, &'a Slice)>`
               found enum `core::option::Option<(&str, &Wtf8)>`
Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `std` (lib) due to 6 previous errors
Build completed unsuccessfully in 0:00:14

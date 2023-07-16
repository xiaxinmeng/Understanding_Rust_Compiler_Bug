plain
    Checking addr2line v0.19.0
error[E0507]: cannot move out of `self.inner` which is behind a shared reference
   --> library/std/src/sys/windows/os_str.rs:103:32
    |
103 |         let (prefix, suffix) = self.inner.into_string_split();
    |                                ^^^^^^^^^^ ------------------- `self.inner` moved due to this method call
    |                                |
    |                                move occurs because `self.inner` has type `Wtf8Buf`, which does not implement the `Copy` trait
    |
note: `Wtf8Buf::into_string_split` takes ownership of the receiver `self`, which moves `self.inner`
   --> library/std/src/sys_common/wtf8.rs:451:30
    |
451 |     pub fn into_string_split(self) -> (String, Wtf8Buf) {
    |                              ^^^^
help: you can `clone` the value and consume it, but this might not be your desired behavior
    |
103 |         let (prefix, suffix) = self.inner.clone().into_string_split();

For more information about this error, try `rustc --explain E0507`.
error: could not compile `std` (lib) due to previous error
Build completed unsuccessfully in 0:00:13

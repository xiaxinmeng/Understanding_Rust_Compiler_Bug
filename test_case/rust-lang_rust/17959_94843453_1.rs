
$ rustc src/bin/main.rs
src/bin/main.rs:14:1: 19:2 error: The requirement `T : core::marker::Sized` is added only by the Drop impl. [E0367]
src/bin/main.rs:14 impl<T> Drop for G<T> {
src/bin/main.rs:15     fn drop(&mut self) {
src/bin/main.rs:16         if !self._ptr.is_null() {
src/bin/main.rs:17         }
src/bin/main.rs:18     }
src/bin/main.rs:19 }
src/bin/main.rs:9:1: 11:2 note: The same requirement must be part of the struct/enum definition
src/bin/main.rs:9 struct G<T: ?Sized> {
src/bin/main.rs:10     _ptr: *const T
src/bin/main.rs:11 }
error: aborting due to previous error
$

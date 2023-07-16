
test.rs:3:6: 3:7 error: the trait `core::marker::Sized` is not implemented for the type `[u8]` [E0277]
test.rs:3 fn f(p: Path) {}
               ^
test.rs:3:6: 3:7 help: run `rustc --explain E0277` to see a detailed explanation
test.rs:3:6: 3:7 note: `[u8]` does not have a constant size known at compile-time. Try to use `&[u8]` or `Box<[u8]>`?
test.rs:3 fn f(p: Path) {}
               ^
test.rs:3:6: 3:7 note: required because it appears within the type `std::sys::os_str::Slice`
test.rs:3 fn f(p: Path) {}
               ^
test.rs:3:6: 3:7 note: required because it appears within the type `std::ffi::os_str::OsStr`
test.rs:3 fn f(p: Path) {}
               ^
test.rs:3:6: 3:7 note: required because it appears within the type `std::path::Path`
test.rs:3 fn f(p: Path) {}
               ^
test.rs:3:6: 3:7 note: all local variables must have a statically known size
test.rs:3 fn f(p: Path) {}
               ^

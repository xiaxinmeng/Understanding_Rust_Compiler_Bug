
Error: `Path: Sized` required because all local variables must have a statically known size [E0277]
test.rs:3 fn f(p: Path) {}
               ^
test.rs:3:6: 3:7 help: run `rustc --explain E0277` to see a detailed explanation
test.rs:3:6: 3:7 note: `Path: Sized` does not hold...
test.rs:3:6: 3:7 note: ...as it requires `std::sys::os_str::Slice: Sized`...
test.rs:3:6: 3:7 note: ...as it requires `std::ffi::os_str::OsStr: Sized`...
test.rs:3:6: 3:7 note: ...as it requires `[u8]: Sized`.

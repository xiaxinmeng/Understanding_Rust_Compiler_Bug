
Error: `Path: Sized` does not hold [E0277]
test.rs:3 fn f(p: Path) {}
               ^
test.rs:3:6: 3:7 help: run `rustc --explain E0277` to see a detailed explanation
test.rs:3:6: 3:7 note: all local variables must have a statically known size
test.rs:3:6: 3:7 note: `Path` does not have a constant size known at compile-time
test.rs:3:6: 3:7 note: as it requires `std::sys::os_str::Slice: Sized`
test.rs:3:6: 3:7 note: as it requires `std::ffi::os_str::OsStr: Sized`
test.rs:3:6: 3:7 note: as it requires `[u8]: Sized`

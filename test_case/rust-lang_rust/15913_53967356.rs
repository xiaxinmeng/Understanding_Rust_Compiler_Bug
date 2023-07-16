
rustc: x86_64-w64-mingw32/stage2/test/nativetest-x86_64-w64-mingw32.exe
A:\msys64\home\retep998\rust\src\libnative\io\process.rs:1206:13: 1206:21 warning: unused import, #[warn(unused_imports)] on by default
A:\msys64\home\retep998\rust\src\libnative\io\process.rs:1206         use std::str;
                                                                          ^~~~~~~~
error: internal compiler error: adt::represent_type called on non-ADT type: i32
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', A:\msys64\home\retep998\rust\src\libsyntax\ast_util.rs:784


/home/retep998/rust/mk/tests.mk:404: recipe for target 'x86_64-w64-mingw32/stage2/test/nativetest-x86_64-w64-mingw32.exe' failed
make: *** [x86_64-w64-mingw32/stage2/test/nativetest-x86_64-w64-mingw32.exe] Error 101

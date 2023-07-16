
% rustc -Lx86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib ../src/test/bench/shootout-pidigits.rs -o pidigits
error: internal compiler error: unexpected failure
This message reflects a bug in the Rust compiler. 
We would appreciate a bug report: http://static.rust-lang.org/doc/master/complement-bugreport.html
note: the compiler hit an unexpected failure path. this is a bug
Ok(task 'rustc' failed at 'assertion failed: lib.rlib.is_none()', /Users/pnkfelix/Dev/Mozilla/rust.git/src/librustc/metadata/loader.rs:192
)

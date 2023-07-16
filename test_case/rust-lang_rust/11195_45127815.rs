
> rustc -L/usr/local/lib test.rs
test.rs:1:1: 1:1 error: multiple dylib candidates for `std` found
test.rs:1 fn main() {}
          ^
test.rs:1:1: 1:1 note: candidate #1: /usr/local/lib/rustlib/x86_64-apple-darwin/lib/libstd-59beb4f7-0.11.0-pre.dylib
test.rs:1 fn main() {}
          ^
test.rs:1:1: 1:1 note: candidate #2: /usr/local/lib/libstd-59beb4f7-0.11.0-pre.dylib
test.rs:1 fn main() {}
          ^
test.rs:1:1: 1:1 error: multiple dylib candidates for `libc` found
test.rs:1 fn main() {}
          ^
test.rs:1:1: 1:1 note: candidate #1: /usr/local/lib/rustlib/x86_64-apple-darwin/lib/liblibc-4f9a876d-0.11.0-pre.dylib
test.rs:1 fn main() {}
          ^
test.rs:1:1: 1:1 note: candidate #2: /usr/local/lib/liblibc-4f9a876d-0.11.0-pre.dylib
test.rs:1 fn main() {}
          ^
error: aborting due to 2 previous errors

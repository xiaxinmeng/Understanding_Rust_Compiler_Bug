
../src/libcore/ptr.rs:134:1: 142:2 error: missing documentation for a function
../src/libcore/ptr.rs:134 pub unsafe fn read_and_drop<T>(dest: *mut T) -> T {
../src/libcore/ptr.rs:135     // Copy the data out from `dest`:
../src/libcore/ptr.rs:136     let tmp = read(&*dest);
../src/libcore/ptr.rs:137 
../src/libcore/ptr.rs:138     // Now mark `dest` as dropped:
../src/libcore/ptr.rs:139     write_bytes(dest, mem::POST_DROP_U8, 1);
                          ...
../src/libcore/lib.rs:58:9: 58:21 note: lint level defined here
../src/libcore/lib.rs:58 #![deny(missing_docs)]
                                 ^~~~~~~~~~~~
error: aborting due to previous error
make: *** [x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.core] Error 101
program finished with exit code 2
elapsedTime=35.897091


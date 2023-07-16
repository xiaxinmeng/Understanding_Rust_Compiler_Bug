
test.rs:3:57: 3:86 error: raw pointers cannot be dereferenced in statics [E0396]
test.rs:3         static BLOCK_UNSAFE_SAFE_PTR: &'static isize = &*(0xdeadbeef as *const isize);
                                                                  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error

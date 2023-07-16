
test.rs:5:38: 5:58 error: raw pointers cannot be dereferenced in constants [E0396]
test.rs:5 const SOME_S: &'static S = unsafe { &*(0x100 as *const S) };
                                               ^~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error

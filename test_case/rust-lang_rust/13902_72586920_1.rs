 rust
/t/triage $ rustc foo.rs
foo.rs:6:23: 6:74 error: expected constant: bad operands for binary [E0080]
foo.rs:6     JSVAL_TAG_INT32 = JSVAL_TAG_CLEAR as u32 | (JSVAL_TYPE_INT32 as u32),
                               ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
foo.rs:7:27: 7:82 error: expected constant: bad operands for binary [E0080]
foo.rs:7     JSVAL_TAG_UNDEFINED = JSVAL_TAG_CLEAR as u32 | (JSVAL_TYPE_UNDEFINED as u32),
                                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to 2 previous errors

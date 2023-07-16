 shell
$ rustc main.rs
main.rs:11:14: 11:36 error: illegal cast; cast from fat pointer: `*const str` as `*const Slice<u8>`
main.rs:11     let t2 = t1 as *const Slice<u8>;
                        ^~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error

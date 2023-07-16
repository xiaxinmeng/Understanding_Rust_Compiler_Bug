 shell
$ rustc main.rs
main.rs:3:15: 3:46 error: illegal cast; cast from fat pointer: `*const str` as `*const u8`
main.rs:3     let ptr = string as *const _ as *const u8;
                        ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error

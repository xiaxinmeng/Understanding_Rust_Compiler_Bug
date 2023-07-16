
rustc: x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore

src/libcore/num/dec2flt/mod.rs:150:1: 150:16 error: unused attribute, #[deny(unused_attributes)] on by default

src/libcore/num/dec2flt/mod.rs:150 #[rustfmt_skip]

                                   ^~~~~~~~~~~~~~~

error: aborting due to previous error

make: *** [x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.core] Error 101

The command "make tidy && make check-notidy -j4" exited with 2.


$ make
cfg: version 1.13.0-dev (413ada304 2016-08-18)
cfg: build triple arm-unknown-linux-gnueabihf
cfg: host triples arm-unknown-linux-gnueabihf
cfg: target triples arm-unknown-linux-gnueabihf
cfg: host for arm-unknown-linux-gnueabihf is arm
cfg: os for arm-unknown-linux-gnueabihf is unknown-linux-gnueabihf
cfg: no good valgrind for arm-unknown-linux-gnueabihf
cfg: using CC=gcc (CFG_CC)
cfg: disabling valgrind run-pass tests
rustc: arm-unknown-linux-gnueabihf/stage0/lib/rustlib/arm-unknown-linux-gnueabi$
f/lib/libcore
src/libcore/intrinsics.rs:178:5: 178:37 error: unrecognized intrinsic function: 
`rustc_peek` [E0093]
src/libcore/intrinsics.rs:178     pub fn rustc_peek<T>(_: T) -> T;
                                  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
src/libcore/intrinsics.rs:178:5: 178:37 help: run `rustc --explain E0093` to se$
 a detailed explanation
error: aborting due to previous error
make: *** [/home/alarm/rust/mk/target.mk:216: arm-unknown-linux-gnueabihf/stage$
/lib/rustlib/arm-unknown-linux-gnueabihf/lib/stamp.core] Error 101

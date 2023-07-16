
$ multirust default nightly-2016-03-11
$ multirust add-target nightly-2016-03-11 arm-unknown-linux-gnueabihf
$ cargo build --target arm-unknown-linux-gnueabihf
nix-0.4.2/src/fcntl.rs:102:28: 102:41 error: mismatched types: expected `*const i8`, found `*const u8` [E0308]
nix-0.4.2/src/fcntl.rs:102 unsafe { ffi::open(cstr.as_ptr(), oflags.bits(), mode.bits() as mode_t }
                                              ^~~~~~~~~~~~~

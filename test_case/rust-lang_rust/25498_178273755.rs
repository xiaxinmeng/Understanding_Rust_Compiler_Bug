 console
$ RUST_BACKTRACE=1 multirust run nightly cargo build
   Compiling regex-syntax v0.2.2
   Compiling libc v0.2.6
   Compiling yaml v0.2.0
/Users/nick/.multirust/toolchains/nightly/cargo/registry/src/github.com-88ac128001ac3a9a/yaml-0.2.0/build.rs:3:21: 3:28 error: unresolved import `std::fs::PathExt`. There is no `PathExt` in `std::fs` [E0432]
/Users/nick/.multirust/toolchains/nightly/cargo/registry/src/github.com-88ac128001ac3a9a/yaml-0.2.0/build.rs:3 use std::fs::{File, PathExt};
                                                                                                                                   ^~~~~~~
/Users/nick/.multirust/toolchains/nightly/cargo/registry/src/github.com-88ac128001ac3a9a/yaml-0.2.0/build.rs:3:21: 3:28 help: run `rustc --explain E0432` to see a detailed explanation
error: aborting due to previous error
Build failed, waiting for other jobs to finish...
Could not compile `yaml`.

To learn more, run the command again with --verbose.

$ multirust run nightly rustc --version
rustc 1.8.0-nightly (094c5b0d6 2016-01-31)

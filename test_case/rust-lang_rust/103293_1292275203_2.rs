
[INFO] [stderr] error: failed to run custom build command for `binaryninjacore-sys v0.1.0 (https://github.com/Vector35/binaryninja-api.git?branch=dev#661d7c95)`
[INFO] [stderr] 
[INFO] [stderr] Caused by:
[INFO] [stderr]   process didn't exit successfully: `/opt/rustwide/target/debug/build/binaryninjacore-sys-8a29f4355f98035e/build-script-build` (exit status: 101)
[INFO] [stderr]   --- stdout
[INFO] [stderr]   cargo:rerun-if-changed=../../binaryninjacore.h
[INFO] [stderr] 
[INFO] [stderr]   --- stderr
[INFO] [stderr]   thread 'main' panicked at 'failed to create required symlink: Os { code: 17, kind: AlreadyExists, message: "File exists" }', /opt/rustwide/cargo-home/git/checkouts/binaryninja-api-3c86176688a74ccb/661d7c9/rust/binaryninjacore-sys/build.rs:70:14
[INFO] [stderr]   stack backtrace:

 bash
$ time RUST_LOG=rustc::metadata::loader make -j4 -- VERBOSE=1 TIME_PASSES=1 TIME_LLVM_PASSES=1 'RUSTFLAGS=--verbose -Z verbose -Z print-link-args -Z print-llvm-passes' RUST_BACKTRACE=1
$ time RUST_LOG=rustc::metadata::loader make -j4 -- VERBOSE=1 TIME_PASSES=1 TIME_LLVM_PASSES=1 'RUSTFLAGS=--verbose -Z verbose -Z print-link-args' RUST_BACKTRACE=1
$ time RUST_LOG=rustc::metadata::loader make -j4 -- VERBOSE=1 TIME_PASSES=1 'RUSTFLAGS=--verbose -Z verbose -Z print-link-args' RUST_BACKTRACE=1
$ time RUST_LOG=rustc::metadata::loader make -j4 -- VERBOSE=1 'RUSTFLAGS=--verbose -Z verbose -Z print-link-args' RUST_BACKTRACE=1
$ time RUST_LOG=rustc::metadata::loader make -j4 -- VERBOSE=1 'RUSTFLAGS=--verbose -Z verbose' RUST_BACKTRACE=1



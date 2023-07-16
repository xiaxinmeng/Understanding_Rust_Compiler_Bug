rust
% ./x.py check
Updating only changed submodules
Submodules updated in 0.04 seconds
downloading https://static.rust-lang.org/dist/2019-03-20/rust-std-beta-x86_64-unknown-linux-gnu.tar.gz
##################################################################################################### 100.0%
extracting /home/lzutao/github.com/lzutao/rust-fork/rust/build/cache/2019-03-20/rust-std-beta-x86_64-unknown-linux-gnu.tar.gz
downloading https://static.rust-lang.org/dist/2019-03-20/rustc-beta-x86_64-unknown-linux-gnu.tar.gz
##################################################################################################### 100.0%
extracting /home/lzutao/github.com/lzutao/rust-fork/rust/build/cache/2019-03-20/rustc-beta-x86_64-unknown-linux-gnu.tar.gz
downloading https://static.rust-lang.org/dist/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
##################################################################################################### 100.0%
extracting /home/lzutao/github.com/lzutao/rust-fork/rust/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
   Compiling proc-macro2 v0.4.24
   Compiling unicode-xid v0.1.0
   Compiling ryu v0.2.7
   Compiling libc v0.2.51
   Compiling serde v1.0.82
   Compiling itoa v0.4.3
   Compiling fixedbitset v0.1.9
   Compiling cc v1.0.28
   Compiling cfg-if v0.1.6
   Compiling ordermap v0.3.5
   Compiling build_helper v0.1.0 (/home/lzutao/github.com/lzutao/rust-fork/rust/src/build_helper)
   Compiling lazy_static v0.2.11
   Compiling getopts v0.2.17
   Compiling cmake v0.1.33
   Compiling petgraph v0.4.13
   Compiling quote v0.6.10
   Compiling num_cpus v1.8.0
   Compiling filetime v0.2.4
   Compiling time v0.1.40
   Compiling serde_json v1.0.33
   Compiling toml v0.4.10
   Compiling syn v0.15.22
   Compiling serde_derive v1.0.81
   Compiling bootstrap v0.0.0 (/home/lzutao/github.com/lzutao/rust-fork/rust/src/bootstrap)
    Finished dev [unoptimized] target(s) in 1m 28s
Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Checking std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cc v1.0.28
    Checking core v0.0.0 (/home/lzutao/github.com/lzutao/rust-fork/rust/src/libcore)
   Compiling libc v0.2.51
   Compiling unwind v0.0.0 (/home/lzutao/github.com/lzutao/rust-fork/rust/src/libunwind)
   Compiling build_helper v0.1.0 (/home/lzutao/github.com/lzutao/rust-fork/rust/src/build_helper)
   Compiling compiler_builtins v0.1.8
   Compiling cmake v0.1.33
   Compiling backtrace-sys v0.1.27
   Compiling std v0.0.0 (/home/lzutao/github.com/lzutao/rust-fork/rust/src/libstd)
    Checking rustc-std-workspace-core v1.0.0 (/home/lzutao/github.com/lzutao/rust-fork/rust/src/tools/rustc-std-workspace-core)
   Compiling rustc_asan v0.0.0 (/home/lzutao/github.com/lzutao/rust-fork/rust/src/librustc_asan)
   Compiling rustc_lsan v0.0.0 (/home/lzutao/github.com/lzutao/rust-fork/rust/src/librustc_lsan)
   Compiling rustc_tsan v0.0.0 (/home/lzutao/github.com/lzutao/rust-fork/rust/src/librustc_tsan)
   Compiling rustc_msan v0.0.0 (/home/lzutao/github.com/lzutao/rust-fork/rust/src/librustc_msan)
    Checking alloc v0.0.0 (/home/lzutao/github.com/lzutao/rust-fork/rust/src/liballoc)
    Checking rustc-demangle v0.1.10
    Checking panic_abort v0.0.0 (/home/lzutao/github.com/lzutao/rust-fork/rust/src/libpanic_abort)
    Checking panic_unwind v0.0.0 (/home/lzutao/github.com/lzutao/rust-fork/rust/src/libpanic_unwind)
    Finished release [optimized] target(s) in 32.12s
Checking test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Checking rustc_term v0.0.1
    Checking getopts v0.2.17
    Checking proc_macro v0.0.0 (/home/lzutao/github.com/lzutao/rust-fork/rust/src/libproc_macro)
    Checking libtest v0.0.1
error[E0460]: found possibly newer version of crate `std` which `getopts` depends onest
  --> /home/lzutao/.cargo/registry/src/github.com-1ecc6299db9ec823/libtest-0.0.1/lib.rs:34:5
   |
34 | use getopts;
   |     ^^^^^^^
   |
   = note: perhaps that crate needs to be recompiled?
   = note: the following crate versions were found:
           crate `std`: /home/lzutao/github.com/lzutao/rust-fork/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-d26c9396879f81c2.rmeta
           crate `getopts`: /home/lzutao/github.com/lzutao/rust-fork/rust/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/deps/libgetopts-267ef145c76bc717.rmeta

error: aborting due to previous error

For more information about this error, try `rustc --explain E0460`.
error: Could not compile `libtest`.

To learn more, run the command again with --verbose.
command did not execute successfully: "/home/lzutao/github.com/lzutao/rust-fork/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "1" "--release" "--manifest-path" "/home/lzutao/github.com/lzutao/rust-fork/rust/src/libtest/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101
failed to run: /home/lzutao/github.com/lzutao/rust-fork/rust/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:23

</details>


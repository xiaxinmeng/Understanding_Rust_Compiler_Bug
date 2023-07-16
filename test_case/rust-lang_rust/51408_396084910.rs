plain
###########################################################               82.4%
######################################################################## 100.0%
[00:01:14] extracting /checkout/obj/build/cache/2018-05-10/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:14]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:34]     Updating git repository `https://github.com/est31/addr2line`
[00:01:35]     Updating git repository `https://github.com/est31/gimli`
[00:01:36]     Updating git repository `https://github.com/rust-lang-nursery/rustfmt`
[00:01:39]     Updating git repository `https://github.com/gimli-rs/cpp_demangle`
[00:01:41]     Updating git repository `https://github.com/kevinaboos/stable_deref_trait`
[00:01:42]  Downloading filetime v0.1.15
[00:01:42]  Downloading time v0.1.39
[00:01:42]  Downloading petgraph v0.4.12
[00:01:42]  Downloading getopts v0.2.17
---
[00:02:52]  Downloading pretty_assertions v0.5.1
[00:02:52]  Downloading ucd-util v0.1.1
[00:02:52]  Downloading vcpkg v0.2.3
[00:02:52]  Downloading termion v1.5.1
[00:02:52]  Downloading scroll v0.9.0
[00:02:52]  Downloading uuid v0.6.5
[00:02:52]  Downloading goblin v0.0.15
[00:02:53]  Downloading error-chain v0.11.0
[00:02:53]  Downloading open v1.2.1
[00:02:53]  Downloading tempfile v3.0.2
[00:02:53]  Downloading clap v2.31.2
---
[00:03:43]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:03:43]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:03:43]    Compiling plain v0.2.3
[00:03:45]    Compiling nodrop v0.1.12
[00:03:45]    Compiling stable_deref_trait v1.0.0 (https://github.com/kevinaboos/stable_deref_trait?rev=8aa58b57a20b0dc215084f84de81e0f7e7dea76e#8aa58b57)
[00:03:45]    Compiling fallible-iterator v0.1.4
[00:03:45]    Compiling byteorder v1.2.2
[00:03:46]    Compiling smallvec v0.4.4
[00:03:46]    Compiling lazycell v1.0.0
---
[00:03:47]    Compiling proc-macro2 v0.4.6
[00:03:47]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:03:48]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:03:51]    Compiling arrayvec v0.4.7
[00:03:51]    Compiling cpp_demangle v0.2.9 (https://github.com/gimli-rs/cpp_demangle?rev=6d5320d2f5c119923647b6d7d2a7d563717a4ddf#6d5320d2)
[00:03:51]    Compiling intervaltree v0.2.3
[00:03:52]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:03:52]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:03:54]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:03:55]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:03:55]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:03:55]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:03:55]    Compiling gimli v0.16.0 (https://github.com/est31/gimli?rev=304caacf8b73f117813b1a0d436ce24403459bf2#304caacf)
[00:03:57]    Compiling quote v0.6.3
[00:03:58]    Compiling syn v0.14.2
[00:04:19]    Compiling scroll_derive v0.9.4
[00:04:21]    Compiling scroll v0.9.0
[00:04:21]    Compiling scroll v0.9.0
[00:04:22]    Compiling goblin v0.0.15
[00:04:37]    Compiling object v0.9.0
[00:04:39]    Compiling addr2line v0.6.0 (https://github.com/est31/addr2line?branch=no_std_prototype#19fca3a5)
[00:04:46] warning: function is never used: `get_executable_filename`
[00:04:46]    --> libstd/sys/unix/backtrace/mod.rs:101:5
[00:04:46]     |
[00:04:46] 101 |     pub fn get_executable_filename() -> io::Result<(Vec<c_char>, fs::File)> {
[00:04:46]     |
[00:04:46]     = note: #[warn(dead_code)] on by default
[00:04:46] 
[00:05:03]     Finished release [optimized] target(s) in 1m 19.80s
---
[00:21:09] Building stage0 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
[00:21:09]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:21:09]    Compiling rustc_codegen_llvm v0.0.0 (file:///checkout/src/librustc_codegen_llvm)
[00:21:09]    Compiling cc v1.0.15
[00:21:09] error[E0460]: found possibly newer version of crate `arrayvec` which `std` depends on
[00:21:09]   |
[00:21:09]   = note: perhaps that crate needs to be recompiled?
[00:21:09]   = note: the following crate versions were found:
[00:21:09]           crate `arrayvec`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarrayvec-723190bddfb7f7c4.rlib
[00:21:09]           crate `arrayvec`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libarrayvec-723190bddfb7f7c4.rlib
[00:21:09]           crate `std`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-ccc26bb68f7390fd.so
[00:21:09]           crate `std`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-ccc26bb68f7390fd.rlib
[00:21:09] error: aborting due to previous error
[00:21:09] 
[00:21:09] For more information about this error, try `rustc --explain E0460`.
[00:21:09] error: Could not compile `cc`.
[00:21:09] error: Could not compile `cc`.
[00:21:09] 
[00:21:09] Caused by:
[00:21:09]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name cc /cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.15/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info

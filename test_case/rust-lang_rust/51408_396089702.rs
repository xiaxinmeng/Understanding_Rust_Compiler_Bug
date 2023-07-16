plain
###################################                                       48.7%
######################################################################## 100.0%
[00:01:02] extracting /checkout/obj/build/cache/2018-05-10/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:02]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:24]     Updating git repository `https://github.com/est31/addr2line`
[00:01:34]     Updating git repository `https://github.com/est31/gimli`
[00:01:47]     Updating git repository `https://github.com/rust-lang-nursery/rustfmt`
[00:01:56]     Updating git repository `https://github.com/gimli-rs/cpp_demangle`
[00:01:57]     Updating git repository `https://github.com/kevinaboos/stable_deref_trait`
[00:01:58]  Downloading lazy_static v0.2.11
[00:01:58]  Downloading filetime v0.1.15
[00:01:58]  Downloading toml v0.4.6
[00:01:58]  Downloading cc v1.0.15
---
[00:03:13]  Downloading rustc-ap-serialize v149.0.0
[00:03:13]  Downloading rustc-ap-rustc_data_structures v149.0.0
[00:03:13]  Downloading bitflags v0.9.1
[00:03:13]  Downloading markup5ever v0.7.2
[00:03:13]  Downloading goblin v0.0.15
[00:03:14]  Downloading scroll v0.9.0
[00:03:14]  Downloading uuid v0.6.5
[00:03:15]  Downloading walkdir v2.1.4
[00:03:15]  Downloading crossbeam v0.3.2
[00:03:15]  Downloading same-file v1.0.2
[00:03:16]  Downloading thread_local v0.3.5
---
[00:04:10]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:04:10]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:04:10]    Compiling plain v0.2.3
[00:04:12]    Compiling nodrop v0.1.12
[00:04:12]    Compiling stable_deref_trait v1.0.0 (https://github.com/kevinaboos/stable_deref_trait?rev=8aa58b57a20b0dc215084f84de81e0f7e7dea76e#8aa58b57)
[00:04:12]    Compiling byteorder v1.2.2
[00:04:13]    Compiling fallible-iterator v0.1.4
[00:04:13]    Compiling libc v0.0.0 (file:///checkout/src/rustc/libc_shim)
[00:04:13]    Compiling lazycell v1.0.0
---
[00:04:14]    Compiling proc-macro2 v0.4.6
[00:04:14]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:04:15]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:04:16]    Compiling arrayvec v0.4.7
[00:04:17]    Compiling cpp_demangle v0.2.9 (https://github.com/gimli-rs/cpp_demangle?rev=6d5320d2f5c119923647b6d7d2a7d563717a4ddf#6d5320d2)
[00:04:21]    Compiling intervaltree v0.2.3
[00:04:21]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:04:21]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:04:21]    Compiling gimli v0.16.0 (https://github.com/est31/gimli?rev=304caacf8b73f117813b1a0d436ce24403459bf2#304caacf)
[00:04:22]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:04:23]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:04:23]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:04:24]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:04:24]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:04:24]    Compiling quote v0.6.3
[00:04:26]    Compiling syn v0.14.2
[00:04:47]    Compiling scroll_derive v0.9.4
[00:04:49]    Compiling scroll v0.9.0
[00:04:50]    Compiling goblin v0.0.15
[00:05:05]    Compiling object v0.9.0
[00:05:07]    Compiling addr2line v0.6.0 (https://github.com/est31/addr2line?branch=no_std_prototype#19fca3a5)
[00:05:15] warning: function is never used: `get_executable_filename`
[00:05:15]    --> libstd/sys/unix/backtrace/mod.rs:101:5
[00:05:15]     |
[00:05:15] 101 |     pub fn get_executable_filename() -> io::Result<(Vec<c_char>, fs::File)> {
[00:05:15]     |
[00:05:15]     = note: #[warn(dead_code)] on by default
[00:05:15] 
[00:05:32]     Finished release [optimized] target(s) in 1m 21.76s

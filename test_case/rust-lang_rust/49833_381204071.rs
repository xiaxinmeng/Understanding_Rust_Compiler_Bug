plain
[00:00:46] configure: rust.quiet-tests     := True
---
[00:07:52] error[E0433]: failed to resolve. Use of undeclared type or module `Entry`
[00:07:52]    --> librustc/ty/maps/on_disk_cache.rs:842:13
[00:07:52]     |
[00:07:52] 842 |             Entry::Occupied(e) => e.get(),
[00:07:52]     |             ^^^^^ Use of undeclared type or module `Entry`
[00:07:52]
[00:07:52] error[E0433]: failed to resolve. Use of undeclared type or module `Entry`
[00:07:52]    --> librustc/ty/maps/on_disk_cache.rs:843:13
[00:07:52]     |
[00:07:52] 843 |             Entry::Vacant(e) => {
[00:07:52]     |             ^^^^^ Use of undeclared type or module `Entry`
[00:07:52]
[00:07:55] error: unused import: `FxHashSet`
[00:07:55]   --> librustc/ty/maps/on_disk_cache.rs:19:44
[00:07:55]    |
[00:07:55] 19 | use rustc_data_structures::fx::{FxHashMap, FxHashSet};
[00:07:55]    |                                            ^^^^^^^^^
[00:07:55]    |
[00:07:55]    = note: `-D unused-imports` implied by `-D warnings`
[00:07:55]
pos-9f3518d56a01456f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c04ded78717d5d67.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c04ded78717d5d67.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-2803512e03c19cf7.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-3e35efb9e5bc7a9a.rlib --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-895f7ddc4467bb8d.so --extern rustc_back=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_back-861dba9fe03aa669.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-9866e194db82a141.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-93cb1ddd29ab61a4.so --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-3c8223b0152f22a5.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-ca61dfec7c40c4d4.rlib --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-65dde0349e75e965.rlib --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-6e4119b5ec8457a3.so --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-3706e912fdb98df1.so --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-5bcc8c1ccd509892.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-f2778bd6cd1c5bdd.so --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-109fed6e8125a798.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-8b35e3c2ea935fab/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-63734d0048644b22/out` (exit code: 101)
[00:08:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:08:32] expected success, got: exit code: 101
[00:08:32] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
---
121748 ./obj/build/bootstrap/debug/incremental/bootstrap-351vorei3hhuv/s-f032dm7esi-tltjm4-3ijxj2oqwtm6x

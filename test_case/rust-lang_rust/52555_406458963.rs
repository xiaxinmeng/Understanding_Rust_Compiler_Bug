plain
[00:04:33]     Checking rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:04:46]     Checking proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:04:46]     Checking syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:05:06] error[E0308]: mismatched types
[00:05:06]    --> librustc/ty/query/job.rs:288:25
[00:05:06]     |
[00:05:06] 288 |     if visited.contains(&query.as_ptr()) {
[00:05:06]     |                         ^^^^^^^^^^^^^^^ expected struct `rustc_data_structures::ptr_key::PtrKey`, found *-ptr
[00:05:06]     |
[00:05:06]     = note: expected type `&rustc_data_structures::ptr_key::PtrKey<'_, ty::query::job::QueryJob<'tcx>>`
[00:05:06]                found type `&*const ty::query::job::QueryJob<'_>`
[00:05:06] error[E0308]: mismatched types
[00:05:06] error[E0308]: mismatched types
[00:05:06]    --> librustc/ty/query/job.rs:303:20
[00:05:06]     |
[00:05:06] 303 |     visited.insert(query.as_ptr());
[00:05:06]     |                    ^^^^^^^^^^^^^^ expected struct `rustc_data_structures::ptr_key::PtrKey`, found *-ptr
[00:05:06]     |
[00:05:06]     = note: expected type `rustc_data_structures::ptr_key::PtrKey<'tcx, ty::query::job::QueryJob<'tcx>>`
[00:05:06]                found type `*const ty::query::job::QueryJob<'_>`
[00:05:06] error[E0308]: mismatched types
[00:05:06] error[E0308]: mismatched types
[00:05:06]    --> librustc/ty/query/job.rs:328:25
[00:05:06]     |
[00:05:06] 328 |     if visited.contains(&query.as_ptr()) {
[00:05:06]     |                         ^^^^^^^^^^^^^^^ expected struct `rustc_data_structures::ptr_key::PtrKey`, found *-ptr
[00:05:06]     |
[00:05:06]     = note: expected type `&rustc_data_structures::ptr_key::PtrKey<'_, ty::query::job::QueryJob<'tcx>>`
[00:05:06]                found type `&*const ty::query::job::QueryJob<'_>`
[00:05:07] error[E0308]: mismatched types
[00:05:07] error[E0308]: mismatched types
[00:05:07]    --> librustc/ty/query/job.rs:337:20
[00:05:07]     |
[00:05:07] 337 |     visited.insert(query.as_ptr());
[00:05:07]     |                    ^^^^^^^^^^^^^^ expected struct `rustc_data_structures::ptr_key::PtrKey`, found *-ptr
[00:05:07]     |
[00:05:07]     = note: expected type `rustc_data_structures::ptr_key::PtrKey<'tcx, ty::query::job::QueryJob<'tcx>>`
[00:05:07]                found type `*const ty::query::job::QueryJob<'_>`
[00:05:07] error[E0308]: mismatched types
[00:05:07] error[E0308]: mismatched types
[00:05:07]    --> librustc/ty/query/job.rs:401:51
[00:05:07]     |
[00:05:07] 401 |             if connected_to_root(query.1.clone(), &mut visited) {
[00:05:07]     |                                                   ^^^^^^^^^^^^ expected struct `rustc_data_structures::ptr_key::PtrKey`, found *-ptr
[00:05:07]     |
[00:05:07]     = note: expected type `&mut std::collections::HashSet<rustc_data_structures::ptr_key::PtrKey<'_, ty::query::job::QueryJob<'_>>>`
[00:05:07]                found type `&mut std::collections::HashSet<*const ty::query::job::QueryJob<'_>, _>`
[00:05:09] error: aborting due to 5 previous errors
[00:05:09] 
[00:05:09] For more information about this error, try `rustc --explain E0308`.
[00:05:09] error: Could not compile `rustc`.
[00:05:09] error: Could not compile `rustc`.
[00:05:09] 
[00:05:09] Caused by:
[00:05:09]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,metadata -C prefer-dynamic -C opt-level=2 -C metadata=e1ec367da7d5d6bf -C extra-filename=-e1ec367da7d5d6bf --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-3ce234ae04ed760b.rmeta --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-e7e3a0157dbc76ee.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-5099f2622d0ac9cb.rmeta --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-208ff00e15aeee7f.rmeta --extern chalk_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_engine-29f487680208f92a.rmeta --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-83d6969cb02f5efc.rmeta --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-410c921ac9ec05c1.rmeta --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-e6141df6f7164f9b.rmeta --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-5fa0c601664c62aa.rmeta --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-0a76c86914261754.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-93b7c86a2fe6817d.rmeta --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-f56562cb4c559f82.rmeta --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-76f5481ef511e806.rmeta --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-6ea937467f532a8c.rmeta --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-87e7db216964a7cd.rmeta --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-1a80c54d7987acf6.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-4a7b93a63f774f75.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-09fea6f08ef4ba92.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-593062ac6cf2bf39.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-03e7d42fc96ee656.rmeta --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-fe69fd07d1e5383f.rmeta --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f17568cb588a2773.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-319daba453093775.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-01970413ab530b41.rmeta --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-1238cf2992abc0a0.rmeta -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-7205e6adb9d66e6b/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-27571ee36438df44/out` (exit code: 101)

[00:05:09] travis_time:end:stage0-rustc:start=1532044373763532796,finish=1532044430166183956,duration=56402651160


[00:05:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:05:09] expected success, got: exit code: 101
[00:05:09] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:05:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
[00:05:09] Build completed unsuccessfully in 0:03:43
travis_time:end:115ae503:start=1532044120593580679,finish=1532044430381583046,duration=309788002367

---
travis_time:end:14ed297a:start=1532044430658198550,finish=1532044430665451446,duration=7252896
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1662dcd2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ed338b8
travis_time:start:0ed338b8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06c81351
$ dmesg | grep -i kill

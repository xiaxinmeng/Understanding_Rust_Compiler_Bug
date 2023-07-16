plain
[00:03:22]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:03:22]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:03:23]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:03:23]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:03:25] error[E0599]: no variant named `same` found for type `unicode::mapping_table::LookupInner` in the current scope
[00:03:25]   --> libcore/unicode/mapping_table.rs:22:28
[00:03:25]    |
[00:03:25] 22 |             None => Lookup(LookupInner::same(c)),
[00:03:25]    |                            ^^^^^^^^^^^^^^^^^ variant not found in `unicode::mapping_table::LookupInner`
[00:03:25] ...
[00:03:25] 35 | pub enum LookupInner {
[00:03:25]    | -------------------- variant `same` not found here
[00:03:25]    = note: did you mean `variant::Same`?
[00:03:25] 
[00:03:25] error[E0308]: mismatched types
[00:03:25]   --> libcore/unicode/mapping_table.rs:25:52
[00:03:25]   --> libcore/unicode/mapping_table.rs:25:52
[00:03:25]    |
[00:03:25] 25 |                 match s.iter().rposition(|&c| c == 0) {
[00:03:25]    |                                                    ^ expected char, found u8
[00:03:25] 
[00:03:25] error[E0599]: no associated item named `Same` found for type `unicode::mapping_table::Lookup` in the current scope
[00:03:25]   --> libcore/unicode/mapping_table.rs:50:13
[00:03:25]    |
[00:03:25] 42 | pub struct Lookup(LookupInner);
[00:03:25]    | ------------------------------- associated item `Same` not found for this
[00:03:25] ...
[00:03:25] 50 |             Lookup::Same(c) => {
[00:03:25]    |             ^^^^^^^^^^^^^^^ associated item not found in `unicode::mapping_table::Lookup`
[00:03:25] 
[00:03:25] error[E0599]: no associated item named `Iter` found for type `unicode::mapping_table::Lookup` in the current scope
[00:03:25]   --> libcore/unicode/mapping_table.rs:54:13
[00:03:25]    |
[00:03:25] 42 | pub struct Lookup(LookupInner);
[00:03:25]    | ------------------------------- associated item `Iter` not found for this
[00:03:25] ...
[00:03:25] 54 |             Lookup::Iter(iter) => iter.next(),
[00:03:25]    |             ^^^^^^^^^^^^^^^^^^ associated item not found in `unicode::mapping_table::Lookup`
[00:03:25] 
[00:03:25] error[E0599]: no associated item named `Iter` found for type `unicode::mapping_table::Lookup` in the current scope
[00:03:25]   --> libcore/unicode/mapping_table.rs:51:25
[00:03:25]    |
[00:03:25] 42 | pub struct Lookup(LookupInner);
[00:03:25]    | ------------------------------- associated item `Iter` not found for this
[00:03:25] ...
[00:03:25] 51 |                 *self = Lookup::Iter([].iter());
[00:03:25]    |                         ^^^^^^^^^^^^ associated item not found in `unicode::mapping_table::Lookup`
[00:03:25] 
[00:03:25] error[E0599]: no associated item named `Same` found for type `unicode::mapping_table::Lookup` in the current scope
[00:03:25]   --> libcore/unicode/mapping_table.rs:61:13
[00:03:25]    |
[00:03:25] 42 | pub struct Lookup(LookupInner);
[00:03:25]    | ------------------------------- associated item `Same` not found for this
[00:03:25] ...
[00:03:25] 61 |             Lookup::Same(_) => (1, Some(1)),
[00:03:25]    |             ^^^^^^^^^^^^^^^ associated item not found in `unicode::mapping_table::Lookup`
[00:03:25] 
[00:03:25] error[E0599]: no associated item named `Iter` found for type `unicode::mapping_table::Lookup` in the current scope
[00:03:25]   --> libcore/unicode/mapping_table.rs:62:13
[00:03:25]    |
[00:03:25] 42 | pub struct Lookup(LookupInner);
[00:03:25]    | ------------------------------- associated item `Iter` not found for this
[00:03:25] ...
[00:03:25] 62 |             Lookup::Iter(iter) => iter.size_hint(),
[00:03:25]    |             ^^^^^^^^^^^^^^^^^^ associated item not found in `unicode::mapping_table::Lookup`
[00:03:25] error: aborting due to 7 previous errors
[00:03:25] 
[00:03:25] Some errors occurred: E0308, E0599.
[00:03:25] For more information about an error, try `rustc --explain E0308`.
[00:03:25] For more information about an error, try `rustc --explain E0308`.
[00:03:25] error: Could not compile `core`.
[00:03:25] 
[00:03:25] Caused by:
[00:03:25]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=0d1ebef792b1d9ca -C extra-filename=-0d1ebef792b1d9ca --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 101)
[00:03:25] warning: build failed, waiting for other jobs to finish...
[00:03:35] error: build failed
[00:03:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:35] expected success, got: exit code: 101
[00:03:35] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:35] travis_fold:end:stage0-std

[00:03:35] travis_time:end:stage0-std:start=1526844149130064434,finish=1526844175583372211,duration=26453307777


[00:03:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:35] Build completed unsuccessfully in 0:00:27
[00:03:35] Makefile:79: recipe for target 'tidy' failed
[00:03:35] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:002961fc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)

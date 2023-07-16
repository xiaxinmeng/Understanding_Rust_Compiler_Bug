plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0730916c
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:22:41]    Compiling parking_lot_core v0.3.0
[01:22:41]    Compiling tempfile v3.0.3
[01:22:42]    Compiling parking_lot v0.6.4
[01:22:43]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[01:22:46] error[E0599]: no method named `clean` found for type `rustc_data_structures::indexed_vec::IndexVec<rustc_target::abi::VariantIdx, rustc::ty::VariantDef>` in the current scope
[01:22:46]    --> librustdoc/clean/inline.rs:235:48
[01:22:46]     |
[01:22:46] 235 |         variants: cx.tcx.adt_def(did).variants.clean(cx),
[01:22:46]     |
[01:22:46]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:46]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:46]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[01:22:46]             candidate #1: `clean::Clean`
[01:22:48] error: aborting due to previous error
[01:22:48] 
[01:22:48] For more information about this error, try `rustc --explain E0599`.
[01:22:48] error: Could not compile `rustdoc`.
---
travis_time:end:003cff00:start=1541101261514348547,finish=1541101261522700780,duration=8352233
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:001bb847
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04cd8e05
travis_time:start:04cd8e05
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03a7244f
$ dmesg | grep -i kill

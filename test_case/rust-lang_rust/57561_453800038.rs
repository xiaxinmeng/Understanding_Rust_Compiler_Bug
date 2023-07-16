plain
[01:32:15]     |                                       ^^^^^^^^^^
[01:32:15] 
[01:33:15] [RUSTC-TIMING] racer test:false 65.267
[01:33:40] [RUSTC-TIMING] rustfmt_nightly test:false 90.581
[01:33:42] error[E0433]: failed to resolve: could not find `MakeGlobMap` in `rustc_resolve`
[01:33:42]     |
[01:33:42]     |
[01:33:42] 349 |         result.make_glob_map = rustc_resolve::MakeGlobMap::Yes;
[01:33:42]     |                                               ^^^^^^^^^^^ could not find `MakeGlobMap` in `rustc_resolve`
[01:33:42] 
[01:33:44] error[E0609]: no field `make_glob_map` on type `build::rustc::rustc_driver::driver::CompileController<'_>`
[01:33:44]     |
[01:33:44]     |
[01:33:44] 349 |         result.make_glob_map = rustc_resolve::MakeGlobMap::Yes;
[01:33:44]     |
[01:33:44]     |
[01:33:44]     = note: available fields are: `after_parse`, `after_expand`, `after_hir_lowering`, `after_analysis`, `compilation_done` ... and 4 others
[01:33:45] error: aborting due to 2 previous errors
[01:33:45] 
[01:33:45] Some errors occurred: E0433, E0609.
[01:33:45] For more information about an error, try `rustc --explain E0433`.
---
travis_time:end:126b697a:start=1547351710255101469,finish=1547351710266468757,duration=11367288
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0da011fb
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:003a933b
travis_time:start:003a933b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1e76ff26
$ dmesg | grep -i kill

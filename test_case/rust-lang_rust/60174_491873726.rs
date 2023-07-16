plain
[00:07:25]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:44] error[E0308]: mismatched types
[00:07:44]     --> src/librustc/hir/map/mod.rs:1425:59
[00:07:44]      |
[00:07:44] 1425 |             format!("arm {}{}", map.node_to_pretty_string(id), id_str)
[00:07:44]      |                                                           ^^ expected struct `syntax::ast::NodeId`, found struct `hir::HirId`
[00:07:44]      = note: expected type `syntax::ast::NodeId`
[00:07:44]                 found type `hir::HirId`
[00:07:44] 
[00:07:52] [RUSTC-TIMING] syntax_ext test:false 26.408
---
travis_time:end:0946165e:start=1557761746831555360,finish=1557761746842375141,duration=10819781
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11d2857d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:21ac5432
travis_time:start:21ac5432
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0675e620
$ dmesg | grep -i kill

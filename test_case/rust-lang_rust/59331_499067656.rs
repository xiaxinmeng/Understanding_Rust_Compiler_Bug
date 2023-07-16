plain
[00:07:38]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:41] error[E0433]: failed to resolve: use of undeclared type or module `TypeVariableOrigin`
[00:07:41]    --> src/librustc/infer/error_reporting/need_type_info.rs:125:24
[00:07:41]     |
[00:07:41] 125 |                 if let TypeVariableOrigin::TypeParameterDefinition(_, name) =
[00:07:41]     |                        ^^^^^^^^^^^^^^^^^^ use of undeclared type or module `TypeVariableOrigin`
[00:08:07] [RUSTC-TIMING] syntax_ext test:false 28.771
[00:08:11] error: aborting due to previous error
[00:08:11] 
[00:08:11] For more information about this error, try `rustc --explain E0433`.
---
travis_time:end:1f58dc20:start=1559738752451478294,finish=1559738752460398688,duration=8920394
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0267cdd2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:178392a8
travis_time:start:178392a8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0d225eb7
$ dmesg | grep -i kill

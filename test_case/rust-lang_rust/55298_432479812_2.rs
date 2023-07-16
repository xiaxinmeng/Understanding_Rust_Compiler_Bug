\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs","byte_start":1209,"byte_end":1220,"line_start":41,"line_end":41,"column_start":23,"column_end":34,"is_primary":true,"text":[{"text":"                   _: TokenStream) -> Box<MacResult+'cx> {","highlight_start":23,"highlight_end":34}],"label":"expected 5 parameters, found 4","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`expand` from trait: `fn(&Self, &'cx mut syntax::ext::base::ExtCtxt<'_>, syntax_pos::Span, syntax::tokenstream::TokenStream, std::option::Option<syntax_pos::Span>) -> std::boxed::Box<(dyn syntax::ext::base::MacResult + 'cx)>`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0050]: method `expand` has 4 parameters but the declaration in trait `syntax::ext::base::TTMacroExpander::expand` has 5\n  --> /checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.wn-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:10:05] 
[01:10:05] 
[01:10:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:05] Build completed unsuccessfully in 0:24:25
[01:10:05] Build completed unsuccessfully in 0:24:25
[01:10:05] Makefile:58: recipe for target 'check' failed
[01:10:05] make: *** [check] Error 1
3051832 ./obj
2839780 ./obj/build
2207984 ./obj/build/x86_64-unknown-linux-gnu
1197692 ./.git
---
travis_time:end:1bed80ee:start=1540344727825447141,finish=1540344727831442621,duration=5995480
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01b7d570
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16dae0a0
travis_time:start:16dae0a0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08855535
$ dmesg | grep -i kill

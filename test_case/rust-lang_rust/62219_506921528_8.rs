\n\nIf the item you are importing is not defined in some super-module of the\ncurrent module, then it must also be declared as public (e.g., `pub fn`).\n"},"level":"error","spans":[{"file_name":"tests/ui/invalid_ref.rs","byte_start":1095,"byte_end":1101,"line_start":44,"line_end":44,"column_start":43,"column_end":49,"is_primary":true,"text":[{"text":"    let ref_uninit: &T = std::intrinsics::uninit(); // warning","highlight_start":43,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"a function with a similar name exists","code":null,"level":"help","spans":[{"file_name":"tests/ui/invalid_ref.rs","byte_start":1095,"byte_end":1101,"line_start":44,"line_end":44,"column_start":43,"column_end":49,"is_primary":true,"text":[{"text":"    let ref_uninit: &T = std::intrinsics::uninit(); // warning","highlight_start":43,"highlight_end":49}],"label":null,"suggested_replacement":"init","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0425]: cannot find function `uninit` in module `std::intrinsics`\n  --> tests/ui/invalid_ref.rs:44:43\n   |\nLL |     let ref_uninit: &T = std::intrinsics::uninit(); // warning\n   |                                           ^^^^^^ help: a function with a similar name exists: `init`\n\n"}
[01:30:39] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:30:39] {"message":"Some errors have detailed explanations: E0425, E0432.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors have detailed explanations: E0425, E0432.\n"}
[01:30:39] 
[01:30:39] ------------------------------------------
[01:30:39] 
[01:30:39] thread '[ui] ui/invalid_ref.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
---
travis_time:end:007a188c:start=1561777576018316920,finish=1561777576026641796,duration=8324876
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:375222b4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:024a547d
travis_time:start:024a547d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:228ff6c0
$ dmesg | grep -i kill

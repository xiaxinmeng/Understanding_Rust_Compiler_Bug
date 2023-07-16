\n"},"level":"error","spans":[{"file_name":"tests/run-pass/async-fn.rs","byte_start":925,"byte_end":931,"line_start":39,"line_end":39,"column_start":61,"column_end":67,"is_primary":true,"text":[{"text":"    assert_eq!(unsafe { Pin::new_unchecked(&mut fut) }.poll(&waker), Poll::Ready(31));","highlight_start":61,"highlight_end":67}],"label":"types differ in mutability","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `&mut std::task::Context<'_>`\n   found type `&std::task::Waker`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> tests/run-pass/async-fn.rs:39:61\n   |\n39 |     assert_eq!(unsafe { Pin::new_unchecked(&mut fut) }.poll(&waker), Poll::Ready(31));\n   |                                                             ^^^^^^ types differ in mutability\n   |\n   = note: expected type `&mut std::task::Context<'_>`\n              found type `&std::task::Waker`\n\n"}
[01:34:45] {"message":"For more information about this error, try `rustc --explain E0308`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0308`.\n"}
[01:34:45] 
[01:34:45] ------------------------------------------
[01:34:45] 
---
travis_time:end:002ca29a:start=1554502304395177780,finish=1554502304407462331,duration=12284551
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:194445c0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0575ad08
travis_time:start:0575ad08
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1a4b0cb0
$ dmesg | grep -i kill

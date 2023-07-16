plain
[00:04:49]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:05:07] error[E0599]: no method named `name` found for type `&syntax::ast::Attribute` in the current scope
[00:05:07]    --> src/librustc/lint/mod.rs:727:34
[00:05:07]     |
[00:05:07] 727 |         if Level::from_str(&attr.name().as_str()).is_some() {
[00:05:07]     |
[00:05:07]     = help: items from traits can only be used if the trait is implemented and in scope
[00:05:07]     = help: items from traits can only be used if the trait is implemented and in scope
[00:05:07]     = note: the following traits define an item `name`, perhaps you need to implement one of them:
[00:05:07]             candidate #1: `hir::map::Named`
[00:05:07]             candidate #2: `lint::LintPass`
[00:05:17] error: aborting due to previous error
[00:05:17] 
[00:05:17] For more information about this error, try `rustc --explain E0599`.
[00:05:17] error: Could not compile `rustc`.
---
travis_time:end:01fdff6c:start=1552722084938913096,finish=1552722084944241781,duration=5328685
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2019a84b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0081343c
travis_time:start:0081343c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:239208f8
$ dmesg | grep -i kill

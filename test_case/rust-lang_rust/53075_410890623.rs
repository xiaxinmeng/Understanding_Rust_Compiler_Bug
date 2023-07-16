plain
[01:00:09] ....................................................................................................
[01:00:11] ....................................................................................................
[01:00:14] ....................................................................................................
[01:00:17] ....................................................................................................
[01:00:19] ......iiiiiiiii.....................................................................................
[01:00:25] ....................................................................................................
[01:00:29] ...........i........................................................................................
[01:00:32] ....................i...............................................................................
[01:00:35] ....................................................................................................
---
[01:16:25] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:16:25]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[01:16:37] error[E0599]: no method named `sample` found for type `rand::distributions::Uniform<u32>` in the current scope
[01:16:37]   --> libcore/../libcore/tests/num/flt2dec/random.rs:77:42
[01:16:37]    |
[01:16:37] 77 |         let x = f32::from_bits(f32_range.sample(&mut rng));
[01:16:37]    |
[01:16:37]    = help: items from traits can only be used if the trait is in scope
[01:16:37]    = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:16:37]            `use rand::distributions::Distribution;`
[01:16:37]            `use rand::distributions::Distribution;`
[01:16:37] 
[01:16:37] error[E0599]: no method named `ind_sample` found for type `rand::distributions::Uniform<u64>` in the current scope
[01:16:37]   --> libcore/../libcore/tests/num/flt2dec/random.rs:88:42
[01:16:37]    |
[01:16:37] 88 |         let x = f64::from_bits(f64_range.ind_sample(&mut rng));
[01:16:37]    |
[01:16:37]    = help: items from traits can only be used if the trait is in scope
[01:16:37]    = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:16:37]    = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:16:37]            `use rand::distributions::IndependentSample;`
or directory
travis_time:end:0e2efe1c:start=1533600349580410994,finish=1533600349588018716,duration=7607722
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_fold:start:after_failure.4
travis_time:start:2573d03a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:036336fc
travis_time:start:036336fc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:006f9321
$ dmesg | grep -i kill

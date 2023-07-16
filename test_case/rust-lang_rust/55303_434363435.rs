plain
[01:41:50] test slice::sort_unstable ... ok
[01:41:50] 
[01:41:50] failures:
[01:41:50] 
[01:41:50] ---- num::flt2dec::strategy::dragon::shortest_sanity_test stdout ----
[01:41:50] thread 'num::flt2dec::strategy::dragon::shortest_sanity_test' panicked at 'shortest mismatch for v=ldexp_f32(1.0, 25): actual ("2", 1), expected ("33554432", 8)', libcore/../libcore/tests/num/flt2dec/mod.rs:249:5
[01:41:50] ---- num::flt2dec::strategy::dragon::test_to_exact_exp_str stdout ----
[01:41:50] thread 'num::flt2dec::strategy::dragon::test_to_exact_exp_str' panicked at 'assertion failed: `(left == right)`
[01:41:50] thread 'num::flt2dec::strategy::dragon::test_to_exact_exp_str' panicked at 'assertion failed: `(left == right)`
[01:41:50]   left: `"0e0"`,
[01:41:50]  right: `"1e-45"`', libcore/../libcore/tests/num/flt2dec/mod.rs:806:5
[01:41:50] ---- num::flt2dec::strategy::dragon::test_to_exact_fixed_str stdout ----
[01:41:50] thread 'num::flt2dec::strategy::dragon::test_to_exact_fixed_str' panicked at 'assertion failed: `(left == right)`
[01:41:50] thread 'num::flt2dec::strategy::dragon::test_to_exact_fixed_str' panicked at 'assertion failed: `(left == right)`
[01:41:50]   left: `"0.0000000000000000000000000000000000000000000000000000000000000000"`,
[01:41:50]  right: `"0.0000000000000000000000000000000000000000000014012984643248170709"`', libcore/../libcore/tests/num/flt2dec/mod.rs:1059:5
[01:41:50] ---- num::flt2dec::strategy::dragon::test_to_shortest_exp_str stdout ----
[01:41:50] thread 'num::flt2dec::strategy::dragon::test_to_shortest_exp_str' panicked at 'assertion failed: `(left == right)`
[01:41:50]   left: `"0"`,
[01:41:50]   left: `"0"`,
[01:41:50]  right: `"1e-45"`', libcore/../libcore/tests/num/flt2dec/mod.rs:662:5
[01:41:50] ---- num::flt2dec::strategy::dragon::test_to_shortest_str stdout ----
[01:41:50] thread 'num::flt2dec::strategy::dragon::test_to_shortest_str' panicked at 'assertion failed: `(left == right)`
[01:41:50]   left: `"0"`,
[01:41:50]   left: `"0"`,
[01:41:50]  right: `"0.000000000000000000000000000000000000000000001"`', libcore/../libcore/tests/num/flt2dec/mod.rs:548:5
[01:41:50] ---- num::flt2dec::strategy::grisu::shortest_sanity_test stdout ----
[01:41:50] ---- num::flt2dec::strategy::grisu::shortest_sanity_test stdout ----
[01:41:50] thread 'num::flt2dec::strategy::grisu::shortest_sanity_test' panicked at 'shortest mismatch for v=ldexp_f32(1.0, 25): actual ("2", 1), expected ("33554432", 8)', libcore/../libcore/tests/num/flt2dec/mod.rs:249:5
[01:41:50] ---- num::flt2dec::strategy::grisu::test_to_exact_exp_str stdout ----
[01:41:50] thread 'num::flt2dec::strategy::grisu::test_to_exact_exp_str' panicked at 'assertion failed: `(left == right)`
[01:41:50] thread 'num::flt2dec::strategy::grisu::test_to_exact_exp_str' panicked at 'assertion failed: `(left == right)`
[01:41:50]   left: `"0e0"`,
[01:41:50]  right: `"1e-45"`', libcore/../libcore/tests/num/flt2dec/mod.rs:806:5
[01:41:50] ---- num::flt2dec::strategy::grisu::test_to_exact_fixed_str stdout ----
[01:41:50] thread 'num::flt2dec::strategy::grisu::test_to_exact_fixed_str' panicked at 'assertion failed: `(left == right)`
[01:41:50] thread 'num::flt2dec::strategy::grisu::test_to_exact_fixed_str' panicked at 'assertion failed: `(left == right)`
[01:41:50]   left: `"0.0000000000000000000000000000000000000000000000000000000000000000"`,
[01:41:50]  right: `"0.0000000000000000000000000000000000000000000014012984643248170709"`', libcore/../libcore/tests/num/flt2dec/mod.rs:1059:5
[01:41:50] ---- num::flt2dec::strategy::grisu::test_to_shortest_exp_str stdout ----
[01:41:50] thread 'num::flt2dec::strategy::grisu::test_to_shortest_exp_str' panicked at 'assertion failed: `(left == right)`
[01:41:50]   left: `"0"`,
[01:41:50]   left: `"0"`,
[01:41:50]  right: `"1e-45"`', libcore/../libcore/tests/num/flt2dec/mod.rs:662:5
[01:41:50] ---- num::flt2dec::strategy::grisu::test_to_shortest_str stdout ----
[01:41:50] thread 'num::flt2dec::strategy::grisu::test_to_shortest_str' panicked at 'assertion failed: `(left == right)`
[01:41:50]   left: `"0"`,
[01:41:50]   left: `"0"`,
[01:41:50]  right: `"0.000000000000000000000000000000000000000000001"`', libcore/../libcore/tests/num/flt2dec/mod.rs:548:5
[01:41:50] ---- num::flt2dec::strategy::dragon::exact_sanity_test stdout ----
[01:41:50] ---- num::flt2dec::strategy::dragon::exact_sanity_test stdout ----
[01:41:50] thread 'num::flt2dec::strategy::dragon::exact_sanity_test' panicked at 'expected finite, got Zero instead', libcore/../libcore/tests/num/flt2dec/mod.rs:31:25
[01:41:50] ---- num::flt2dec::strategy::grisu::exact_sanity_test stdout ----
[01:41:50] ---- num::flt2dec::strategy::grisu::exact_sanity_test stdout ----
[01:41:50] thread 'num::flt2dec::strategy::grisu::exact_sanity_test' panicked at 'expected finite, got Zero instead', libcore/../libcore/tests/num/flt2dec/mod.rs:31:25
[01:41:50] 
[01:41:50] failures:
[01:41:50]     num::flt2dec::strategy::dragon::exact_sanity_test
[01:41:50]     num::flt2dec::strategy::dragon::shortest_sanity_test
---
[01:41:50] 
[01:41:50] error: test failed, to rerun pass '--test coretests'
[01:41:50] 
[01:41:50] 
[01:41:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "arm-linux-androideabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--"
[01:41:50] 
[01:41:50] 
[01:41:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-linux-androideabi
[01:41:50] Build completed unsuccessfully in 1:29:49
---
travis_time:end:33b9b4f4:start=1540915487073391944,finish=1540915487090646981,duration=17255037
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04aa7923
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ac2f7d6
travis_time:start:0ac2f7d6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01f7ed2a
$ dmesg | grep -i kill

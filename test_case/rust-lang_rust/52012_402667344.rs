plain
[01:10:47] test [ui] ui/needless_range_loop.rs ... ok
[01:10:47] test [ui] ui/needless_update.rs ... ok
[01:10:47] test [ui] ui/needless_pass_by_value_proc_macro.rs ... ok
[01:10:48] test [ui] ui/neg_multiply.rs ... ok
[01:10:48] test [ui] ui/neg_cmp_op_on_partial_ord.rs ... ok
[01:10:48] test [ui] ui/new_without_default.rs ... ok
[01:10:48] test [ui] ui/no_effect.rs ... ok
[01:10:48] test [ui] ui/non_copy_const.rs ... ok
[01:10:49] test [ui] ui/non_expressive_names.rs ... ok
---
[01:11:00] test result: ok. 200 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
[01:11:00] 
[01:11:00] 
[01:11:00] running 1 test
[01:11:00] test [ui] ui-toml/toml_unknown_key/conf_unknown_key.rs ... ok
[01:11:00] test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
[01:11:00] 
[01:11:00] 
[01:11:00] running 1 test
[01:11:00] running 1 test
[01:11:01] test [ui] ui-toml/bad_toml/conf_bad_toml.rs ... ok
[01:11:01] test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
[01:11:01] 
[01:11:01] 
[01:11:01] running 1 test
[01:11:01] running 1 test
[01:11:01] test [ui] ui-toml/bad_toml_type/conf_bad_type.rs ... ok
[01:11:01] test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
[01:11:01] 
[01:11:01] 
[01:11:01] running 1 test
[01:11:01] running 1 test
[01:11:01] test [ui] ui-toml/toml_trivially_copy/test.rs ... ok
[01:11:01] test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
[01:11:01] 
[01:11:01] 
[01:11:01] running 1 test
[01:11:01] running 1 test
[01:11:02] test [ui] ui-toml/toml_blacklist/conf_french_blacklisted_name.rs ... ok
[01:11:02] test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
[01:11:02] 
[01:11:02] test compile_test ... ok
[01:11:02] 
---
travis_time:end:0522ff30:start=1530782728354485529,finish=1530782728362171769,duration=7686240
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:13ed1c6e
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:106019a8
$ dmesg | grep -i kill

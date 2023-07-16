plain
[01:08:18] test [ui] ui/needless_range_loop.rs ... ok
[01:08:18] test [ui] ui/needless_update.rs ... ok
[01:08:19] test [ui] ui/needless_pass_by_value_proc_macro.rs ... ok
[01:08:19] test [ui] ui/neg_multiply.rs ... ok
[01:08:19] test [ui] ui/neg_cmp_op_on_partial_ord.rs ... ok
[01:08:19] test [ui] ui/new_without_default.rs ... ok
[01:08:19] test [ui] ui/no_effect.rs ... ok
[01:08:19] test [ui] ui/non_copy_const.rs ... ok
[01:08:20] test [ui] ui/non_expressive_names.rs ... ok
---
[01:08:31] test result: ok. 200 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
[01:08:31] 
[01:08:31] 
[01:08:31] running 1 test
[01:08:31] test [ui] ui-toml/toml_unknown_key/conf_unknown_key.rs ... ok
[01:08:31] test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
[01:08:31] 
[01:08:31] 
[01:08:31] running 1 test
[01:08:31] running 1 test
[01:08:31] test [ui] ui-toml/bad_toml/conf_bad_toml.rs ... ok
[01:08:31] test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
[01:08:31] 
[01:08:31] 
[01:08:31] running 1 test
[01:08:31] running 1 test
[01:08:32] test [ui] ui-toml/bad_toml_type/conf_bad_type.rs ... ok
[01:08:32] test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
[01:08:32] 
[01:08:32] 
[01:08:32] running 1 test
[01:08:32] running 1 test
[01:08:32] test [ui] ui-toml/toml_trivially_copy/test.rs ... ok
[01:08:32] test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
[01:08:32] 
[01:08:32] 
[01:08:32] running 1 test
[01:08:32] running 1 test
[01:08:32] test [ui] ui-toml/toml_blacklist/conf_french_blacklisted_name.rs ... ok
[01:08:32] test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
[01:08:32] 
[01:08:32] test compile_test ... ok
[01:08:32] 
---
travis_time:end:0ecd43c2:start=1530693790150683577,finish=1530693790158235258,duration=7551681
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0583c492
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1aa43cc2
$ dmesg | grep -i kill

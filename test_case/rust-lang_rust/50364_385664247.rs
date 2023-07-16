plain
[01:07:40] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:40]    Compiling core v0.0.0 (file:///checkout/src/libcore)
unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:08:07] 
[01:08:07] 
[01:08:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:07] Build completed unsuccessfully in 0:28:20
[01:08:07] Build completed unsuccessfully in 0:28:20
[01:08:07] Makefile:58: recipe for target 'check' failed
[01:08:07] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00fefb50
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)

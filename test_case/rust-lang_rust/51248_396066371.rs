plain
[01:15:35] 
[01:15:35] To learn more, run the command again with --verbose.
[01:15:35] 
[01:15:35] 
[01:15:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:15:35] 
[01:15:35] 
[01:15:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:35] Build completed unsuccessfully in 0:35:35
[01:15:35] Build completed unsuccessfully in 0:35:35
[01:15:35] make: *** [check] Error 1
[01:15:35] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1f2c5400
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)

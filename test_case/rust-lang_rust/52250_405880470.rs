plain
[01:07:26] 
[01:07:26] To learn more, run the command again with --verbose.
[01:07:26] 
[01:07:26] 
[01:07:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_data_structures" "--" "--quiet"
[01:07:26] 
[01:07:26] 
[01:07:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:26] Build completed unsuccessfully in 0:26:00
[01:07:26] Build completed unsuccessfully in 0:26:00
[01:07:26] make: *** [check] Error 1
[01:07:26] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:16254381
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)

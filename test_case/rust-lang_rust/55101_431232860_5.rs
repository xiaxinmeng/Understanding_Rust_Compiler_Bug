\n\nIf you're using a stable or a bet] 18 
[00:49:39] - error[E0645]: trait aliases are not yet implemented (see issue #41517)
[00:49:39] -    |
[00:49:39] -    |
[00:49:39] - LL | trait WithWhere<Art, Thou> = Romeo + Romeo where Fore<(Art, Thou)>: Romeo; //~ERROR E0645
[00:49:39] + error: aborting due to previous error
[00:49:39] 24 
[00:49:39] 24 
[00:49:39] - error[E0645]: trait aliases are not yet implemented (see issue #41517)
[00:49:39] -    |
[00:49:39] -    |
[00:49:39] - LL | trait BareWhere<Wild, Are> = where The<Wild>: Things<Are>; //~ERROR E0645
[00:49:39] - 
[00:49:39] - 
[00:49:39] - error[E0645]: trait aliases are not yet implemented (see issue #41517)
[00:49:39] -    |
[00:49:39] -    |
[00:49:39] - LL | trait CD = Clone + Default; //~ERROR E0645
[00:49:39] - 
[00:49:39] - error: aborting due to 6 previous errors
[00:49:39] - 
[00:49:39] - For more information about this error, try `rustc --explain E0645`.
[00:49:39] - For more information about this error, try `rustc --explain E0645`.
[00:49:39] + For more information about this error, try `rustc --explain E0283`.
[00:49:39] 40 
[00:49:39] 
[00:49:39] 
[00:49:39] The actual stderr differed from the expected stderr.
[00:49:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias/trait-alias.stderr
[00:49:39] To update references, rerun the tests and pass the `--bless` flag
[00:49:39] To only update this specific test, also pass `--test-args traiu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:49:39] 
[00:49:39] 
[00:49:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:49:39] Build completed unsuccessfully in 0:03:39
[00:49:39] Build completed unsuccessfully in 0:03:39
[00:49:39] Makefile:58: recipe for target 'check' failed
[00:49:39] make: *** [check] Error 1

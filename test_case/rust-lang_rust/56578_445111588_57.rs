\n\nA trait object is defined over a single, fully-defined trait. With a regular\ndefault parameter, this parameter can just be substituted in. However, if the\ndefault parameter is `Self`, the trait changes for each concrete type; i.e.\n`i32` will be expected to implement `A<i32>`, `bool` will be expected to\nimplement `A<bool>`, etc... These types will not share an implementation of a\nfully-definull,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0393, E0599.\n"}
[00:49:07] {"message":"For more information about an error, try `rustc --explain E0393`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0393`.\n"}
[00:49:07] ------------------------------------------
[00:49:07] 
[00:49:07] thread '[ui] ui/unspecified-self-in-trait-ref.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3284:9
[00:49:07] 
---
[00:49:07] 
[00:49:07] 
[00:49:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:49:07] Build completed unsuccessfully in 0:03:58
[00:49:07] Makefile:58: recipe for target 'check' failed
[00:49:07] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00448f5f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Dec  7 03:31:09 UTC 2018
---
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:08f1d906
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1b054897
$ dmesg | grep -i kill


---- [run-pass] run-pass/issue-24227.rs stdout ----

	

error: compilation failed!

status: exit code: 101

command: /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc /Users/travis/build/rust-lang/rust/src/test/run-pass/issue-24227.rs -L /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-pass --target=x86_64-apple-darwin --error-format json -L /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-pass/issue-24227.stage2-x86_64-apple-darwin.run-pass.libaux -C prefer-dynamic -o /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-pass/issue-24227.stage2-x86_64-apple-darwin -Crpath -O -Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers

stdout:

------------------------------------------

------------------------------------------

stderr:

------------------------------------------

{"message":"linking with `cc` failed: exit code: 254","code":null,"level":"error","spans":[],"children":[{"message":"\"cc\" \"-m64\" \"-L\" \"/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib\" \"/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-pass/issue-24227.0.o\" \"-o\" \"/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-pass/issue-24227.stage2-x86_64-apple-darwin\" \"-Wl,-dead_strip\" \"-nodefaultlibs\" \"-L\" \"/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-pass\" \"-L\" \"/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-pass/issue-24227.stage2-x86_64-apple-darwin.run-pass.libaux\" \"-L\" \"/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers\" \"-L\" \"/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib\" \"-L\" \"/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib\" \"-l\" \"std-babe254b64e6389b\" \"/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-a156453e4a4418dc.rlib\" \"-l\" \"System\" \"-l\" \"pthread\" \"-l\" \"c\" \"-l\" \"m\" \"-Wl,-rpath,@loader_path/../../stage2/lib/rustlib/x86_64-apple-darwin/lib\" \"-Wl,-rpath,/usr/local/lib/rustlib/x86_64-apple-darwin/lib\"","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"ld: warning: directory not found for option '-L/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-pass/issue-24227.stage2-x86_64-apple-darwin.run-pass.libaux'\nclang: error: unable to execute command: Segmentation fault: 11\nclang: error: linker command failed due to signal (use -v to see invocation)\n","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":null}

{"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":null}

------------------------------------------

thread '[run-pass] run-pass/issue-24227.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2575

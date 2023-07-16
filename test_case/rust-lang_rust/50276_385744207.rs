plain
[00:38:18]    Compiling rand v0.4.2
[00:38:25]    Compiling tempdir v0.3.7
[00:38:26]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:40:15]    Compiling rustdoc-tool v0.0.0 (file:///checkout/src/tools/rustdoc)
[00:40:16] error: cannot satisfy dependencies so `getopts` only shows up once
[00:40:16]   |
[00:40:16]   = help: having upstream crates all available in one format will likely make this go away
[00:40:16] error: aborting due to previous error
[00:40:16] 
[00:40:16] error: Could not compile `rustdoc-tool`.
[00:40:16] 
---
[00:40:16] 
[00:40:16] 
[00:40:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:40:16] Build completed unsuccessfully in 0:35:05
[00:40:16] Makefile:28: recipe for target 'all' failed
[00:40:16] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e52c540
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
149124 ./src/llvm-emscripten/test
144712 ./obj/build/bootstrap/debug/incremental
127972 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
123732 ./obj/build/bootstrap/debug/incremental/bootstrap-1wl4zjaz72e5d
123728 ./obj/build/bootstrap/debug/incremental/bootstrap-1wl4zjaz72e5d/s-f0mxbsejbb-vji1yf-1yqdmshn3n7qj
122508 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
102816 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
89936 ./obj/build/x86_64-unknown-linux-gnu/stage1
89912 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib

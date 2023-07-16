plain
[00:38:41]    Compiling rand v0.4.2
[00:38:48]    Compiling tempdir v0.3.7
[00:38:48]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:40:26]    Compiling rustdoc-tool v0.0.0 (file:///checkout/src/tools/rustdoc)
[00:40:27] error: cannot satisfy dependencies so `getopts` only shows up once
[00:40:27]   |
[00:40:27]   = help: having upstream crates all available in one format will likely make this go away
[00:40:27] error: aborting due to previous error
[00:40:27] 
[00:40:27] error: Could not compile `rustdoc-tool`.
[00:40:27] 
---
[00:40:27] 
[00:40:27] 
[00:40:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:40:27] Build completed unsuccessfully in 0:35:27
[00:40:27] make: *** [all] Error 1
[00:40:27] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0fa45ea8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)

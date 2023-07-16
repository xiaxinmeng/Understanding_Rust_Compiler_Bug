

---- [run-pass] run-pass-fulldeps/plugin-args-1.rs stdout ----

error: auxiliary build of "/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/auxiliary/plugin_args.rs" failed to compile: 
status: exit code: 101
command: i686-apple-darwin/stage2/bin/rustc /Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/auxiliary/plugin_args.rs -L i686-apple-darwin/test/run-pass-fulldeps --target=i686-apple-darwin --crate-type=dylib -L i686-apple-darwin/test/run-pass-fulldeps/plugin-args-1.stage2-i686-apple-darwinlibaux -C prefer-dynamic --out-dir i686-apple-darwin/test/run-pass-fulldeps/plugin-args-1.stage2-i686-apple-darwinlibaux --cfg rtopt --cfg debug -O -L i686-apple-darwin/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/auxiliary/plugin_args.rs:49:9: 49:53 error: this function takes 3 parameters but 2 parameters were supplied [E0061]
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/auxiliary/plugin_args.rs:49         NormalTT(box Expander { args: args, }, None));
                                                                                                            ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error

------------------------------------------

thread '[run-pass] run-pass-fulldeps/plugin-args-1.rs' panicked at 'explicit panic', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/compiletest/runtest.rs:1466


---- [run-pass] run-pass-fulldeps/plugin-args-2.rs stdout ----

error: auxiliary build of "/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/auxiliary/plugin_args.rs" failed to compile: 
status: exit code: 101
command: i686-apple-darwin/stage2/bin/rustc /Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/auxiliary/plugin_args.rs -L i686-apple-darwin/test/run-pass-fulldeps --target=i686-apple-darwin --crate-type=dylib -L i686-apple-darwin/test/run-pass-fulldeps/plugin-args-2.stage2-i686-apple-darwinlibaux -C prefer-dynamic --out-dir i686-apple-darwin/test/run-pass-fulldeps/plugin-args-2.stage2-i686-apple-darwinlibaux --cfg rtopt --cfg debug -O -L i686-apple-darwin/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/auxiliary/plugin_args.rs:49:9: 49:53 error: this function takes 3 parameters but 2 parameters were supplied [E0061]
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/auxiliary/plugin_args.rs:49         NormalTT(box Expander { args: args, }, None));
                                                                                                            ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error

------------------------------------------

thread '[run-pass] run-pass-fulldeps/plugin-args-2.rs' panicked at 'explicit panic', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/compiletest/runtest.rs:1466


---- [run-pass] run-pass-fulldeps/plugin-args-3.rs stdout ----

error: auxiliary build of "/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/auxiliary/plugin_args.rs" failed to compile: 
status: exit code: 101
command: i686-apple-darwin/stage2/bin/rustc /Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/auxiliary/plugin_args.rs -L i686-apple-darwin/test/run-pass-fulldeps --target=i686-apple-darwin --crate-type=dylib -L i686-apple-darwin/test/run-pass-fulldeps/plugin-args-3.stage2-i686-apple-darwinlibaux -C prefer-dynamic --out-dir i686-apple-darwin/test/run-pass-fulldeps/plugin-args-3.stage2-i686-apple-darwinlibaux --cfg rtopt --cfg debug -O -L i686-apple-darwin/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/auxiliary/plugin_args.rs:49:9: 49:53 error: this function takes 3 parameters but 2 parameters were supplied [E0061]
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/auxiliary/plugin_args.rs:49         NormalTT(box Expander { args: args, }, None));
                                                                                                            ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error

------------------------------------------

thread '[run-pass] run-pass-fulldeps/plugin-args-3.rs' panicked at 'explicit panic', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/compiletest/runtest.rs:1466



failures:
    [run-pass] run-pass-fulldeps/plugin-args-1.rs
    [run-pass] run-pass-fulldeps/plugin-args-2.rs
    [run-pass] run-pass-fulldeps/plugin-args-3.rs

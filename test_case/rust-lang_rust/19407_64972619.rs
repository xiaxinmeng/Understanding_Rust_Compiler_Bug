
failures:

---- [pretty] run-pass-fulldeps/issue-13560.rs stdout ----

error: auxiliary build of /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/auxiliary/issue-13560-3.rs failed to compile: 
status: exit code: 101
command: x86_64-apple-darwin/stage2/bin/rustc /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/auxiliary/issue-13560-3.rs -L x86_64-apple-darwin/test/run-pass-fulldeps --target=x86_64-apple-darwin -L x86_64-apple-darwin/test/run-pass-fulldeps/issue-13560.stage2-x86_64-apple-darwinlibaux --out-dir x86_64-apple-darwin/test/run-pass-fulldeps/issue-13560.stage2-x86_64-apple-darwinlibaux --cfg rtopt --cfg debug -O -L x86_64-apple-darwin/rt
stdout:
------------------------------------------
task 'rustc' panicked at 'index out of bounds: the len is 2952 but the index is 2952', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/librbml/lib.rs:156

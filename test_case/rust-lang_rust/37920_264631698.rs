
---- [run-pass] run-pass/issue-37290/main.rs stdout ----
	
error: auxiliary build of "/buildslave/rust-buildbot/slave/auto-linux-musl-64-opt/build/src/test/run-pass/issue-37290/auxiliary/lint.rs" failed to compile: 
status: exit code: 101
command: x86_64-unknown-linux-gnu/stage2/bin/rustc /buildslave/rust-buildbot/slave/auto-linux-musl-64-opt/build/src/test/run-pass/issue-37290/auxiliary/lint.rs -L x86_64-unknown-linux-gnu/test/run-pass/ --target=x86_64-unknown-linux-musl --error-format json --crate-type=lib -L x86_64-unknown-linux-gnu/test/run-pass/issue-37290/main.stage2-x86_64-unknown-linux-musl.run-pass.libaux -C prefer-dynamic --out-dir x86_64-unknown-linux-gnu/test/run-pass/issue-37290/main.stage2-x86_64-unknown-linux-musl.run-pass.libaux -C linker=/musl-x86_64/bin/musl-gcc -C ar=ar --cfg rtopt -C rpath -O -L x86_64-unknown-linux-musl/rt -C prefer-dynamic
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
{"message":"can't find crate for `syntax`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n

command: x86_64-unknown-linux-gnu/stage1/bin/rustc /opt/rust/src/test/run-pass/regions-nullary-variant.rs -L x86_64-unknown-linux-gnu/test/run-pass/ --target=x86_64-unknown-linux-gnu -L x86_64-unknown-linux-gnu/test/run-pass/regions-nullary-variant.stage1-x86_64-unknown-linux-gnulibaux -C prefer-dynamic -o x86_64-unknown-linux-gnu/test/run-pass/regions-nullary-variant.stage1-x86_64-unknown-linux-gnu --cfg rtopt --cfg debug =on -O -L x86_64-unknown-linux-gnu/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: multiple input filenames provided

------------------------------------------

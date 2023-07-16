
---- [pretty] run-pass/cci_capture_clause.rs stdout ----

error: pretty-printed source does not typecheck
status: exit code: 101
command: x86_64-unknown-linux-gnu/stage2/bin/rustc - -Zno-trans --crate-type=lib --target=x86_64-unknown-linux-gnu -L x86_64-unknown-linux-gnu/test/run-pass -L x86_64-unknown-linux-gnu/test/run-pass/cci_capture_clause.stage2-x86_64-unknown-linux-gnulibaux --cfg rtopt --cfg debug -O -L x86_64-unknown-linux-gnu/rt

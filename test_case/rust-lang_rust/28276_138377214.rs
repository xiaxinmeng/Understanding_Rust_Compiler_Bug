

---- [pretty] run-pass/std-sync-right-kind-impls.rs stdout ----

error: pretty-printed source does not typecheck
status: signal: 11
command: x86_64-unknown-linux-gnu/stage2/bin/rustc - -Zno-trans --target=x86_64-unknown-linux-gnu -L x86_64-unknown-linux-gnu/test/run-pass/ -L x86_64-unknown-linux-gnu/test/run-pass/std-sync-right-kind-impls.stage2-x86_64-unknown-linux-gnu.pretty.libaux --cfg rtopt -L x86_64-unknown-linux-gnu/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------

------------------------------------------

thread '[pretty] run-pass/std-sync-right-kind-impls.rs' panicked at 'explicit panic', /home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/compiletest/runtest.rs:1501

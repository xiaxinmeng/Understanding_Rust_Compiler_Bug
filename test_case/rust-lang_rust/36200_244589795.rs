
---- [run-pass] run-pass/issue-33992.rs stdout ----

error: compilation failed!
status: signal: 6
command: /buildslave/rust-buildbot/slave/auto-linux-64-opt-rustbuild/build/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /buildslave/rust-buildbot/slave/auto-linux-64-opt-rustbuild/build/src/test/run-pass/issue-33992.rs -L /buildslave/rust-buildbot/slave/auto-linux-64-opt-rustbuild/build/obj/build/x86_64-unknown-linux-gnu/test/run-pass --target=x86_64-unknown-linux-gnu --error-format json -L /buildslave/rust-buildbot/slave/auto-linux-64-opt-rustbuild/build/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-33992.stage2-x86_64-unknown-linux-gnu.run-pass.libaux -C prefer-dynamic -o /buildslave/rust-buildbot/slave/auto-linux-64-opt-rustbuild/build/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-33992.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/buildslave/rust-buildbot/slave/auto-linux-64-opt-rustbuild/build/obj/build/x86_64-unknown-linux-gnu/rust-test-helpers
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
Should never emit this
UNREACHABLE executed at /buildslave/rust-buildbot/slave/auto-linux-64-opt-rustbuild/build/src/llvm/lib/CodeGen/AsmPrinter/AsmPrinter.cpp:336!

------------------------------------------

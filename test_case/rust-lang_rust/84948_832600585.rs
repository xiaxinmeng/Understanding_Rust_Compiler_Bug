plain
test test::test_workspaces_cwd ... ok

failures:

---- fix::fix_deny_warnings_but_not_others stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo fix --allow-no-vcs`
thread 'fix::fix_deny_warnings_but_not_others' panicked at 'assertion failed: p.read_file(\"src/lib.rs\").contains(\"fn bar() {}\")', src/tools/cargo/tests/testsuite/fix.rs:572:5


failures:
    fix::fix_deny_warnings_but_not_others
---


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/cargo src/tools/cargotest
Build completed unsuccessfully in 0:23:26
make: *** [check-aux] Error 1
Makefile:44: recipe for target 'check-aux' failed

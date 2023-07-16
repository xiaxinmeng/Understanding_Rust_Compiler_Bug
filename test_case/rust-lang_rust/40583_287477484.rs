
---- [pretty] pretty/issue-40469.rs stdout ----

	

error: pretty-printed source does not typecheck

status: exit code: 101

command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc - -Zno-trans --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-40469.pretty-out --target=x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-40469.stage2-x86_64-unknown-linux-gnu.pretty.libaux -Crpath -O -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers

stdout:

------------------------------------------

------------------------------------------

stderr:

------------------------------------------

error: couldn't read "auxiliary/issue_40469.rs": No such file or directory (os error 2)

  --> <anon>:13:1

   |

13 | include! ("auxiliary/issue_40469.rs");

   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

------------------------------------------

thread '[pretty] pretty/issue-40469.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2637

note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:

    [pretty] pretty/issue-40469.rs

test result: FAILED. 2610 passed; 1 failed; 27 ignored; 0 measured

plain



The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestIL5Vsq/concurrency/libc_pthread_cond.stage-id.stderr
To only update this specific test, also pass `--test-args concurrency/libc_pthread_cond.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/concurrency/libc_pthread_cond.rs" "-L" "/tmp/compiletestIL5Vsq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestIL5Vsq/concurrency/libc_pthread_cond.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-isolation" "-Zmiri-check-number-validity" "-L" "/tmp/compiletestIL5Vsq/concurrency/libc_pthread_cond.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
-after wait
-

The actual stdout differed from the expected stdout.
Actual stdout saved to /tmp/compiletestIL5Vsq/concurrency/sync.stage-id.stdout

-warning: thread support is experimental and incomplete: weak memory effects are not emulated.
+thread 'rustc' panicked at 'Miri initialization error: type validation failed: encountered pointer to 0x206c5[alloc50]<40>, but expected initialized plain (non-pointer) bytes', src/tools/miri/src/eval.rs:298:13
+stack backtrace:
---
+end of query stack
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestIL5Vsq/concurrency/sync.stage-id.stderr
To only update this specific test, also pass `--test-args concurrency/sync.rs`

error: 2 errors occurred comparing output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/concurrency/sync.rs" "-L" "/tmp/compiletestIL5Vsq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestIL5Vsq/concurrency/sync.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-isolation" "-Zmiri-check-number-validity" "-L" "/tmp/compiletestIL5Vsq/concurrency/sync.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestIL5Vsq/vec.stage-id.stderr
To only update this specific test, also pass `--test-args vec.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vec.rs" "-L" "/tmp/compiletestIL5Vsq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestIL5Vsq/vec.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-tag-raw-pointers" "-Zmiri-check-number-validity" "-L" "/tmp/compiletestIL5Vsq/vec.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
.......... (60/61)
          (61/61)


/checkout/src/test/rustdoc-gui/search-result-display.goml search-result-display... FAILED
[ERROR] (line 6) TimeoutError: waiting for selector "#titles" failed: timeout 30000ms exceeded: for command `wait-for: "#titles"`
Build completed unsuccessfully in 0:00:44

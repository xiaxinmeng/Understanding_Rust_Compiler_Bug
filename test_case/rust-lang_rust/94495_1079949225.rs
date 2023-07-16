plain
To only update this specific test, also pass `--test-args concurrency/libc_pthread_cond.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/concurrency/libc_pthread_cond.rs" "-L" "/tmp/compiletestMD29BU" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestMD29BU/concurrency/libc_pthread_cond.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-isolation" "-Zmiri-check-number-validity" "-L" "/tmp/compiletestMD29BU/concurrency/libc_pthread_cond.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
To only update this specific test, also pass `--test-args concurrency/sync.rs`

error: 2 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/concurrency/sync.rs" "-L" "/tmp/compiletestMD29BU" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestMD29BU/concurrency/sync.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-isolation" "-Zmiri-check-number-validity" "-L" "/tmp/compiletestMD29BU/concurrency/sync.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestMD29BU/vec.stage-id.stderr
To only update this specific test, also pass `--test-args vec.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vec.rs" "-L" "/tmp/compiletestMD29BU" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestMD29BU/vec.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-tag-raw-pointers" "-Zmiri-check-number-validity" "-L" "/tmp/compiletestMD29BU/vec.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
.......... (60/61)
          (61/61)


/checkout/src/test/rustdoc-gui/search-tab-selection-if-current-is-empty.goml search-tab-selection-if-current-is-empty... FAILED
[ERROR] (line 6) TimeoutError: waiting for selector "#titles" failed: timeout 30000ms exceeded: for command `wait-for: "#titles"`
Build completed unsuccessfully in 0:00:46

plain
To only update this specific test, also pass `--test-args concurrency/libc_pthread_cond.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/concurrency/libc_pthread_cond.rs" "-L" "/tmp/compiletestjW70wG" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestjW70wG/concurrency/libc_pthread_cond.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-isolation" "-Zmiri-check-number-validity" "-L" "/tmp/compiletestjW70wG/concurrency/libc_pthread_cond.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
To only update this specific test, also pass `--test-args concurrency/sync.rs`

error: 2 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/concurrency/sync.rs" "-L" "/tmp/compiletestjW70wG" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestjW70wG/concurrency/sync.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-isolation" "-Zmiri-check-number-validity" "-L" "/tmp/compiletestjW70wG/concurrency/sync.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
To only update this specific test, also pass `--test-args vec.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vec.rs" "-L" "/tmp/compiletestjW70wG" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestjW70wG/vec.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-tag-raw-pointers" "-Zmiri-check-number-validity" "-L" "/tmp/compiletestjW70wG/vec.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
.......... (60/61)
          (61/61)


/checkout/src/test/rustdoc-gui/escape-key.goml escape-key... FAILED
[ERROR] (line 6) TimeoutError: waiting for selector "#search h1" failed: timeout 30000ms exceeded: for command `wait-for: "#search h1" // The search element is empty before the first search `
Build completed unsuccessfully in 0:00:43

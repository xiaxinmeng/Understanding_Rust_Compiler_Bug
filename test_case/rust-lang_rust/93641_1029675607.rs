plain
-hello dup fd
-

The actual stdout differed from the expected stdout.
Actual stdout saved to /tmp/compiletesteoCQLf/fs.stage-id.stdout

-hello dup fd
+error: unsupported operation: can't call foreign function: readdir64
+   --> /checkout/library/std/src/sys/unix/fs.rs:482:33
---
+
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletesteoCQLf/fs.stage-id.stderr
To only update this specific test, also pass `--test-args fs.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/fs.rs" "-L" "/tmp/compiletesteoCQLf" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletesteoCQLf/fs.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-isolation" "-L" "/tmp/compiletesteoCQLf/fs.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletesteoCQLf/portable-simd.stage-id.stderr
To only update this specific test, also pass `--test-args portable-simd.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/portable-simd.rs" "-L" "/tmp/compiletesteoCQLf" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletesteoCQLf/portable-simd.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletesteoCQLf/portable-simd.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
......... (50/59)
.........  (59/59)


/checkout/src/test/rustdoc-gui/trait-sidebar-item-order.goml An exception occured: Failed to launch the browser process!
Inconsistency detected by ld.so: dl-tls.c: 493: _dl_allocate_tls_init: Assertion `listp->slotinfo[cnt].gen <= GL(dl_tls_generation)' failed!


TROUBLESHOOTING: https://github.com/puppeteer/puppeteer/blob/master/docs/troubleshooting.md
== STACKTRACE ==
Error
Error
    at innerRunTestCode (/node-v14.4.0-linux-x64/lib/node_modules/browser-ui-test/src/index.js:468:16)
    at runMicrotasks (<anonymous>)
    at processTicksAndRejections (internal/process/task_queues.js:97:5)

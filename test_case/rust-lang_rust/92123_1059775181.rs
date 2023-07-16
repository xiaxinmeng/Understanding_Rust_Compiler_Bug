plain



The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestTtXhmV/available-concurrency.stage-id.stderr
To only update this specific test, also pass `--test-args available-concurrency.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/available-concurrency.rs" "-L" "/tmp/compiletestTtXhmV" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestTtXhmV/available-concurrency.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestTtXhmV/available-concurrency.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
......... (50/60)
.......... (60/60)


/checkout/src/test/rustdoc-gui/toggled-open-implementations.goml An exception occured: Failed to launch the browser process!
Inconsistency detected by ld.so: dl-tls.c: 493: _dl_allocate_tls_init: Assertion `listp->slotinfo[cnt].gen <= GL(dl_tls_generation)' failed!


TROUBLESHOOTING: https://github.com/puppeteer/puppeteer/blob/master/docs/troubleshooting.md
== STACKTRACE ==
Error
Error
    at innerRunTestCode (/node-v14.4.0-linux-x64/lib/node_modules/browser-ui-test/src/index.js:468:16)
    at runMicrotasks (<anonymous>)
    at processTicksAndRejections (internal/process/task_queues.js:97:5)

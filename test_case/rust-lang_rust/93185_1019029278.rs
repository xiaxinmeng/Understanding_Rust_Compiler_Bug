plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 17d29dcdce9b9e838635eb0adefd9b8b1588410b and d94d0ad92ff008481235c7b31a194677854d1d17
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestdAj7M0/async-fn.stage-id.stderr
To only update this specific test, also pass `--test-args async-fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/async-fn.rs" "-L" "/tmp/compiletestdAj7M0" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestdAj7M0/async-fn.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestdAj7M0/async-fn.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestdAj7M0/generator.stage-id.stderr
To only update this specific test, also pass `--test-args generator.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/generator.rs" "-L" "/tmp/compiletestdAj7M0" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestdAj7M0/generator.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestdAj7M0/generator.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
......... (50/58)
........   (58/58)


/checkout/src/test/rustdoc-gui/trait-sidebar-item-order.goml An exception occured: Failed to launch the browser process!
Inconsistency detected by ld.so: dl-tls.c: 493: _dl_allocate_tls_init: Assertion `listp->slotinfo[cnt].gen <= GL(dl_tls_generation)' failed!


TROUBLESHOOTING: https://github.com/puppeteer/puppeteer/blob/master/docs/troubleshooting.md
== STACKTRACE ==
Error
Error
    at innerRunTestCode (/node-v14.4.0-linux-x64/lib/node_modules/browser-ui-test/src/index.js:457:16)
    at runMicrotasks (<anonymous>)
    at processTicksAndRejections (internal/process/task_queues.js:97:5)


command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:16

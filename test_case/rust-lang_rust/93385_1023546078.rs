plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 21b4a9cfdcbb1e76f4b36b5c3cfd64d627285093 and beb52c1403595077e71d68c2961bc36d1d16309d
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
-hello dup fd
-

The actual stdout differed from the expected stdout.
Actual stdout saved to /tmp/compiletestnJh1gw/fs.stage-id.stdout

-hello dup fd
+error: unsupported operation: can't call foreign function: readdir64
+   --> /checkout/library/std/src/sys/unix/fs.rs:482:33
---
+
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestnJh1gw/fs.stage-id.stderr
To only update this specific test, also pass `--test-args fs.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/fs.rs" "-L" "/tmp/compiletestnJh1gw" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestnJh1gw/fs.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-isolation" "-L" "/tmp/compiletestnJh1gw/fs.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
.......... (50/58)
........   (58/58)


/checkout/src/test/rustdoc-gui/docblock-table-overflow.goml An exception occured: socket hang up
== STACKTRACE ==
Error
    at innerRunTestCode (/node-v14.4.0-linux-x64/lib/node_modules/browser-ui-test/src/index.js:457:16)
    at processTicksAndRejections (internal/process/task_queues.js:97:5)
    at async main (/checkout/src/tools/rustdoc-gui/tester.js:226:13)


command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:17

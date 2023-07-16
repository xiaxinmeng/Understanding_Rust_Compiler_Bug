plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between a00e130dae74a213338e2b095ec855156d8f3d8a and 67aaa87f1c8c37284f30e1d3b06769ca924829dd
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
-hello dup fd
-

The actual stdout differed from the expected stdout.
Actual stdout saved to /tmp/compiletestCZxESy/fs.stage-id.stdout

-hello dup fd
+error: unsupported operation: can't call foreign function: readdir64
+   --> /checkout/library/std/src/sys/unix/fs.rs:482:33
---
+
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestCZxESy/fs.stage-id.stderr
To only update this specific test, also pass `--test-args fs.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/fs.rs" "-L" "/tmp/compiletestCZxESy" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestCZxESy/fs.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-isolation" "-L" "/tmp/compiletestCZxESy/fs.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
.......... (50/59)
........  (59/59)


/checkout/src/test/rustdoc-gui/search-tab-selection-if-current-is-empty.goml search-tab-selection-if-current-is-empty... FAILED
[ERROR] (line 6) TimeoutError: waiting for selector "#titles" failed: timeout 30000ms exceeded: for command `wait-for: "#titles"`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:44

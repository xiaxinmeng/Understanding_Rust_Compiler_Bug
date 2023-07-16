plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 498eeb72f590e518e19746b346be53713689e207 and 2db252f1949dbffcadca70836a8695eabc5c394d
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
-hello dup fd
-

The actual stdout differed from the expected stdout.
Actual stdout saved to /tmp/compiletestlrqQkq/fs.stage-id.stdout

-hello dup fd
+error: unsupported operation: can't call foreign function: readdir64
+   --> /checkout/library/std/src/sys/unix/fs.rs:482:33
---
+
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestlrqQkq/fs.stage-id.stderr
To only update this specific test, also pass `--test-args fs.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/fs.rs" "-L" "/tmp/compiletestlrqQkq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestlrqQkq/fs.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-isolation" "-L" "/tmp/compiletestlrqQkq/fs.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
......... (50/59)
........  (59/59)


/checkout/src/test/rustdoc-gui/sidebar-mobile.goml sidebar-mobile... FAILED
[ERROR] (line 42) Error: Evaluation failed: different Y values: 541.25 != 542.96875: for command `assert-position: (".block.keyword li:nth-child(1)", {"y": 542.96875})`

/checkout/src/test/rustdoc-gui/type-declation-overflow.goml type-declation-overflow... FAILED
[ERROR] (line 34) Error: Evaluation failed: expected `986` for property `scrollWidth` for selector `.mobile-topbar .location`, found `504`: for command `assert-property: (".mobile-topbar .location", {"scrollWidth": "986"})`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:19

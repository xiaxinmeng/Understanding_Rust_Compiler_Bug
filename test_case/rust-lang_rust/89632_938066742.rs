plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 0157cc977fd71297ce73e2f249321f5ba2555d42 and 5e4dce8bccf17e1190be33a4bfcc680944386d0d
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletesteNZoZZ/available-concurrency.stage-id.stderr
To only update this specific test, also pass `--test-args available-concurrency.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/available-concurrency.rs" "-L" "/tmp/compiletesteNZoZZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletesteNZoZZ/available-concurrency.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletesteNZoZZ/available-concurrency.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
....       (44/44)


label-next-to-symbol... FAILED
[ERROR] (line 17) Error: Evaluation failed: different Y values: 787 != 810: for command `compare-elements-position: (".item-left .stab.deprecated", ".item-left .stab.portability", ("y"))`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:17

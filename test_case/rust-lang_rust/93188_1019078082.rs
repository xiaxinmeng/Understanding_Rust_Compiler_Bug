plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between ecf72996eda4f8af19b0ca7235c6f62e0245a313 and ee22be0f35acb4b2b1b4720320eeb510bc17dbae
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletesty1syuV/async-fn.stage-id.stderr
To only update this specific test, also pass `--test-args async-fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/async-fn.rs" "-L" "/tmp/compiletesty1syuV" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletesty1syuV/async-fn.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletesty1syuV/async-fn.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletesty1syuV/generator.stage-id.stderr
To only update this specific test, also pass `--test-args generator.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/generator.rs" "-L" "/tmp/compiletesty1syuV" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletesty1syuV/generator.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletesty1syuV/generator.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
F (50/58)
.................   (58/58)


/checkout/src/test/rustdoc-gui/toggle-docs-mobile.goml toggle-docs-mobile... FAILED
[ERROR] (line 26) Error: Evaluation failed: assert didn't fail: for command `assert-attribute-false: (".top-doc", {"open": ""})`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:19

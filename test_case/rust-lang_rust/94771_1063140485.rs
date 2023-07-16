plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 10dccdc7fcbdc64ee9efe2c1ed975ab8c1d61287 and 7e2b7dafd836002d00d68cff01784402d472f115
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
test [ui] run-pass/u128.rs ... ok
diff of stdout:

-[2, 2] Iter([2, 2], [])
+[2, 2] Iter([core::mem::maybe_uninit::MaybeUninit<alloc::boxed::Box<i32>>, core::mem::maybe_uninit::MaybeUninit<alloc::boxed::Box<i32>>], [])
 

The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /tmp/compiletestKRCIvB/vecdeque.stage-id.stdout
To only update this specific test, also pass `--test-args vecdeque.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vecdeque.rs" "-L" "/tmp/compiletestKRCIvB" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestKRCIvB/vecdeque.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-tag-raw-pointers" "-L" "/tmp/compiletestKRCIvB/vecdeque.stage-id.aux"
------------------------------------------
------------------------------------------
[2, 2] Iter([core::mem::maybe_uninit::MaybeUninit<alloc::boxed::Box<i32>>, core::mem::maybe_uninit::MaybeUninit<alloc::boxed::Box<i32>>], [])

------------------------------------------
stderr:
------------------------------------------
---
.......... (60/61)
.          (61/61)


/checkout/src/test/rustdoc-gui/mobile.goml mobile... FAILED
[ERROR] (line 27) Error: Evaluation failed: assert didn't fail: for command `compare-elements-position-near-false: ("#preferred-light-theme .setting-name", "#preferred-light-theme .choice", {"y": 16})`
Build completed unsuccessfully in 0:00:21

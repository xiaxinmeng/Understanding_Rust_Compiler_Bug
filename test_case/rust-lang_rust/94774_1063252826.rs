plain
test [ui] run-pass/u128.rs ... ok
diff of stdout:

-[2, 2] Iter([2, 2], [])
+[2, 2] Iter([core::mem::maybe_uninit::MaybeUninit<alloc::boxed::Box<i32>>, core::mem::maybe_uninit::MaybeUninit<alloc::boxed::Box<i32>>], [])
 

The actual stdout differed from the expected stdout.
Actual stdout saved to /tmp/compiletestDp9Oan/vecdeque.stage-id.stdout
Actual stdout saved to /tmp/compiletestDp9Oan/vecdeque.stage-id.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args vecdeque.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vecdeque.rs" "-L" "/tmp/compiletestDp9Oan" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestDp9Oan/vecdeque.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-tag-raw-pointers" "-L" "/tmp/compiletestDp9Oan/vecdeque.stage-id.aux"
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

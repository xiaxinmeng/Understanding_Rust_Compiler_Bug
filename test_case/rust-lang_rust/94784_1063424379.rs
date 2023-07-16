`
test [ui] run-pass/vec.rs ... ok
diff of stdout:

-[2, 2] Iter([2, 2], [])
+[2, 2] Iter([core::mem::maybe_uninit::MaybeUninit<alloc::boxed::Box<i32>>, core::mem::maybe_uninit::MaybeUninit<alloc::boxed::Box<i32>>], [])
 Iter([], [])
 

The actual stdout differed from the expected stdout.
Actual stdout saved to /tmp/compiletest20lDZR/vecdeque.stage-id.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args vecdeque.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vecdeque.rs" "-L" "/tmp/compiletest20lDZR" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest20lDZR/vecdeque.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-tag-raw-pointers" "-L" "/tmp/compiletest20lDZR/vecdeque.stage-id.aux"
stdout:
------------------------------------------
[2, 2] Iter([core::mem::maybe_uninit::MaybeUninit<alloc::boxed::Box<i32>>, core::mem::maybe_uninit::MaybeUninit<alloc::boxed::Box<i32>>], [])
Iter([], [])

------------------------------------------
stderr:
------------------------------------------

------------------------------------------

test [ui] run-pass/vecdeque.rs ... FAILED
test [ui] run-pass/float.rs ... ok

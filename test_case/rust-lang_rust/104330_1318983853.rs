plain
---- [ui] src/test/ui/array-slice-vec/box-of-array-of-drop-2.rs stdout ----

error: test run failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "0" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/box-of-array-of-drop-2/a"
--- stdout -------------------------------
uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/box-of-array-of-drop-2/a", waiting for result
--- stderr -------------------------------
thread 'main' panicked at 'client.into_inner() failed with Connection reset by peer (os error 104)', src/tools/remote-test-client/src/main.rs:306:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
------------------------------------------

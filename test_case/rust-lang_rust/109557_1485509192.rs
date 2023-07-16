plain
---- [ui] tests/ui/deriving/deriving-via-extension-enum.rs stdout ----

error: test run failed!
status: exit status: 101
command: RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "0" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deriving/deriving-via-extension-enum/a"
uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deriving/deriving-via-extension-enum/a", waiting for result
------------------------------------------
--- stderr -------------------------------
thread 'main' panicked at 'client.read_exact(&mut header) failed with Connection reset by peer (os error 104)', src/tools/remote-test-client/src/main.rs:310:9

plain
failures:

---- [ui] ui/imports/glob-use-std.rs stdout ----

error: error pattern 'panic works' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "0" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/glob-use-std/a"
--- stdout -------------------------------
uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/glob-use-std/a", waiting for result
------------------------------------------
------------------------------------------
--- stderr -------------------------------
thread 'main' panicked at 'client.read_exact(&mut header) failed with Connection reset by peer (os error 104)', src/tools/remote-test-client/src/main.rs:310:9
------------------------------------------




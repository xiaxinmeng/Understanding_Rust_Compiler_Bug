
[01:47:28] diff output.json /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output.json
[01:47:28] 5c5
[01:47:28] < { "type": "test", "name": "b", "event": "failed", "stdout": "thread 'main' panicked at 'assertion failed: false', f.rs:8:5\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.\n" }
[01:47:28] ---
[01:47:28] > { "type": "test", "name": "b", "event": "failed", "message": "test did not panic as expected" }

plain
[00:37:55]    Compiling serde_json v1.0.15
[00:38:04]    Compiling serde_derive_internals v0.23.1
[00:38:12]    Compiling serde_derive v1.0.40
[00:38:30]    Compiling tidy v0.1.0 (file:///checkout/src/tools/tidy)
/release/deps --extern num_traits=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libnum_traits-32459d1c2ac205a6.rlib --extern tidy=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libtidy-90539d438865aac1.rlib` (exit code: 101)
[00:38:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/unstable-book-gen/Cargo.toml" "--features" "" "--message-format" "json"
[00:38:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:38:38] Build completed unsuccessfully in 0:01:02
[00:38:38] Build completed unsuccessfully in 0:01:02
[00:38:38] make: *** [all] Error 1
[00:38:38] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c02f5aa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
